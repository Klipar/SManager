use std::{collections::HashMap, sync::Arc, time::{Duration, Instant}, net::SocketAddr};
use sqlx::PgPool;
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::{TlsAcceptor, server::TlsStream};
use tokio_util::codec::{Framed, LinesCodec};
use futures::{StreamExt, SinkExt};
use tokio::sync::oneshot;

use anyhow::{Context, Result};
use log::{info, error};

use shared::{db::models::Core, server::{
    connection_context::ConnectionContext,
    endpoint::Endpoint,
    get_hash::get_hash,
    handler_trait::HandlerTrait,
    message::{Message, Status},
}};

use crate::extern_server::{connection_registry::{ConnectionRegistry, OutboundRequest}, tls_helpers::build_tls_config};

const TLS_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);
const READ_TIMEOUT: Duration = Duration::from_secs(60);
const MAX_LINE_LENGTH: usize = 65536;


pub struct Server {
    endpoint: Arc<Endpoint>,
    pub is_active: bool,
    pub handlers: Arc<HashMap<String, Arc<dyn HandlerTrait>>>,
    pool: Arc<PgPool>,
    registry: ConnectionRegistry,
}

impl Server {
    pub fn new(endpoint: Arc<Endpoint>, pool: Arc<PgPool>, registry: ConnectionRegistry) -> Self {
        Self {
            endpoint,
            is_active: false,
            handlers: Arc::new(HashMap::new()),
            pool,
            registry,
        }
    }

    pub fn add_handler(&mut self, name: &str, handler: Arc<dyn HandlerTrait>) {
        Arc::make_mut(&mut self.handlers).insert(name.to_string(), handler);
    }

    pub async fn start_server(mut self) -> Result<()> {
        let listener = TcpListener::bind(format!("{}:{}", self.endpoint.ip, self.endpoint.port))
            .await
            .with_context(|| format!("Failed to bind to {}", self.endpoint))?;

        let tls_config = build_tls_config()?;
        let acceptor = TlsAcceptor::from(tls_config);

        self.is_active = true;
        info!("mTLS Server listening on {}", self.endpoint);

        loop {
            let (socket, addr) = match listener.accept().await {
                Ok(v) => v,
                Err(e) => {
                    error!("Failed to accept connection: {e}");
                    continue;
                }
            };

            tokio::spawn(Self::handle_connection(
                socket,
                addr,
                acceptor.clone(),
                Arc::clone(&self.handlers),
                Arc::clone(&self.pool),
                self.registry.clone(),
            ));
        }
    }

    pub async fn handle_connection(
        socket: TcpStream,
        addr: SocketAddr,
        acceptor: TlsAcceptor,
        handlers: Arc<HashMap<String, Arc<dyn HandlerTrait>>>,
        pool: Arc<PgPool>,
        registry: ConnectionRegistry,
    ) {
        let tls_stream = match perform_tls_handshake(socket, addr, &acceptor).await {
            Some(stream) => stream,
            None => return,
        };

        let ctx = match authenticate_client(&tls_stream, addr, &pool).await {
            Some(ctx) => ctx,
            None => return,
        };

        let (tx, rx) = tokio::sync::mpsc::channel::<OutboundRequest>(100);

        let core_id = match ctx.id {
            Some(id) => id,
            None => {
                error!("No core id found...");
                return;
            }
        };

        if !registry.register(core_id, tx).await {
            error!("Core {} already connected, rejecting new session", core_id);
            return;
        }

        info!("TLS connection established: {}", addr);
        run_message_loop( tls_stream, addr, handlers, ctx, rx ).await;
        registry.unregister(core_id).await;
        info!("Disconnected: {}", addr);
    }
}

async fn perform_tls_handshake(
    socket: TcpStream,
    addr: SocketAddr,
    acceptor: &TlsAcceptor,
) -> Option<tokio_rustls::server::TlsStream<TcpStream>> {
    match tokio::time::timeout(TLS_HANDSHAKE_TIMEOUT, acceptor.accept(socket)).await {
        Ok(Ok(stream)) => Some(stream),
        Ok(Err(e)) => {
            error!("TLS handshake failed for {addr}: {e}");
            None
        }
        Err(_) => {
            error!("TLS handshake timed out for {addr}");
            None
        }
    }
}

async fn authenticate_client(
    stream: &tokio_rustls::server::TlsStream<TcpStream>,
    addr: SocketAddr,
    pool: &PgPool,
) -> Option<ConnectionContext> {
    let client_certs = stream.get_ref().1.peer_certificates()?;
    let cert_der = client_certs.first().or_else(|| {
        error!("Client sent no certificate: {addr}");
        None
    })?;

    let (_, parsed_cert) = x509_parser::parse_x509_certificate(cert_der.as_ref())
        .ok()
        .or_else(|| {
            error!("Failed to parse client certificate from {addr}");
            None
        })?;

    let cn = parsed_cert
        .tbs_certificate
        .subject
        .iter_common_name()
        .next()
        .and_then(|cn| cn.as_str().ok())
        .unwrap_or_default();

    let core = sqlx::query_as::<_, Core>(
        "SELECT * FROM cores WHERE client_hash = $1 AND ip = $2"
    )
    .bind(get_hash(cn))
    .bind(addr.ip().to_string())
    .fetch_one(pool)
    .await;

    match core {
        Ok(core) => {
            info!("Authenticated: `{}`", core.name);
            let mut ctx = ConnectionContext::new(addr.ip().to_string());
            ctx.id = Some(core.id);
            Some(ctx)
        }
        Err(_) => {
            error!("Unauthorized client CN=`{cn}` from {addr}");
            None
        }
    }
}

async fn run_message_loop(
    tls_stream: TlsStream<TcpStream>,
    addr: SocketAddr,
    handlers: Arc<HashMap<String, Arc<dyn HandlerTrait>>>,
    mut ctx: ConnectionContext,
    mut outbound_rx: tokio::sync::mpsc::Receiver<OutboundRequest>,
) {
    let mut framed = Framed::new(
        tls_stream,
        LinesCodec::new_with_max_length(MAX_LINE_LENGTH),
    );

    let mut pending: HashMap<u64, oneshot::Sender<Message>> = HashMap::new();

    let mut last_activity = Instant::now();

    loop {
        tokio::select! {
            incoming = framed.next() => {
                let result = match incoming {
                    Some(r) => r,
                    None => break,
                };

                let line = match result {
                    Ok(l) => l,
                    Err(e) => {
                        error!("Read error from {addr}: {e}");
                        break;
                    }
                };

                last_activity = Instant::now();

                let Ok(msg) = serde_json::from_str::<Message>(&line) else {
                    let _ = send_error(&mut framed, 0, "Failed to parse JSON").await;
                    continue;
                };

                match msg {

                    Message::Request { id, action, data } => {
                        let Some(handler) = handlers.get(&action) else {
                            error!("Unknown action `{action}` from {addr}");
                            let _ = send_error(
                                &mut framed,
                                id,
                                &format!("Unknown action: {action}")
                            ).await;
                            continue;
                        };

                        let mut response = handler.handle(data, &mut ctx).await;
                        response.set_id(id);

                        let json = serde_json::to_string(&response)
                            .expect("Response serialization failed");

                        if let Err(e) = framed.send(json).await {
                            error!("Write error to {addr}: {e}");
                            break;
                        }

                        last_activity = Instant::now();
                    }

                    Message::Response { id, .. } => {
                        if let Some(tx) = pending.remove(&id) {
                            let _ = tx.send(msg);
                        }

                        last_activity = Instant::now();
                    }
                }
            }

            outbound = outbound_rx.recv() => {
                let Some(outbound) = outbound else {
                    break;
                };

                let id = match &outbound.message {
                    Message::Request { id, .. } => *id,
                    _ => continue,
                };

                pending.insert(id, outbound.response_tx);

                let json = serde_json::to_string(&outbound.message)
                    .expect("Outbound serialization failed");

                if let Err(e) = framed.send(json).await {
                    error!("Write error to {addr}: {e}");
                    break;
                }

                last_activity = Instant::now();
            }

            _ = tokio::time::sleep(Duration::from_secs(1)) => {
                if last_activity.elapsed() > READ_TIMEOUT {
                    error!("Idle timeout for {addr}");
                    break;
                }
            }
        }
    }
}

async fn send_error(
    framed: &mut Framed<TlsStream<TcpStream>, LinesCodec>,
    id: u64,
    message: &str,
) -> Result<(), impl std::error::Error> {
    let mut response = Message::new_response(Status::Error, None, 400, message.to_string());
    response.set_id(id);
    framed.send(serde_json::to_string(&response).unwrap()).await
}