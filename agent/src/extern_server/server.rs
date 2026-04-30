use std::{collections::HashMap, sync::Arc, time::Duration, net::SocketAddr};
use sqlx::PgPool;
use tokio::net::{TcpListener, TcpStream};
use tokio_rustls::{TlsAcceptor, server::TlsStream};
use tokio_util::codec::{Framed, LinesCodec};
use futures::{StreamExt, SinkExt};

use anyhow::{Context, Result};
use log::{info, error};

use shared::{db::models::Core, server::{
    connection_context::ConnectionContext,
    endpoint::Endpoint,
    get_hash::get_hash,
    handler_trait::HandlerTrait,
    message::{Message, Status},
}};

use crate::extern_server::tls_helpers::build_tls_config;

const TLS_HANDSHAKE_TIMEOUT: Duration = Duration::from_secs(10);
const READ_TIMEOUT: Duration = Duration::from_secs(60);
const MAX_LINE_LENGTH: usize = 65536;


pub struct Server {
    endpoint: Arc<Endpoint>,
    pub is_active: bool,
    pub handlers: Arc<HashMap<String, Arc<dyn HandlerTrait>>>,
    pool: Arc<PgPool>,
}

impl Server {
    pub fn new(endpoint: Arc<Endpoint>, pool: Arc<PgPool>) -> Self {
        Self {
            endpoint,
            is_active: false,
            handlers: Arc::new(HashMap::new()),
            pool,
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
            ));
        }
    }

    pub async fn handle_connection(
        socket: TcpStream,
        addr: SocketAddr,
        acceptor: TlsAcceptor,
        handlers: Arc<HashMap<String, Arc<dyn HandlerTrait>>>,
        pool: Arc<PgPool>,
    ) {
        let tls_stream = match perform_tls_handshake(socket, addr, &acceptor).await {
            Some(stream) => stream,
            None => return,
        };

        let ctx = match authenticate_client(&tls_stream, addr, &pool).await {
            Some(ctx) => ctx,
            None => return,
        };

        info!("TLS connection established: {}", addr);
        run_message_loop(tls_stream, addr, handlers, ctx).await;
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
) {
    let mut framed = Framed::new(tls_stream, LinesCodec::new_with_max_length(MAX_LINE_LENGTH));

    loop {
        let result = match tokio::time::timeout(READ_TIMEOUT, framed.next()).await {
            Ok(Some(r)) => r,
            Ok(None) => break,
            Err(_) => {
                error!("Read timeout for {addr}");
                break;
            }
        };

        let line = match result {
            Ok(l) => l,
            Err(e) => {
                error!("Read error from {addr}: {e}");
                break;
            }
        };

        let Ok(msg) = serde_json::from_str::<Message>(&line) else {
            if let Err(e) = send_error(&mut framed, 0, "Failed to parse JSON").await {
                error!("Write error to {addr}: {e}");
                break;
            }
            continue;
        };

        match msg {
            Message::Request { id, action, data } => {
                let Some(handler) = handlers.get(&action) else {
                    error!("Unknown action `{action}` from {addr}");
                    if let Err(e) = send_error(&mut framed, id, &format!("Unknown action: {action}")).await {
                        error!("Write error to {addr}: {e}");
                        break;
                    }
                    continue;
                };

                let mut response = handler.handle(data, &mut ctx).await;
                response.set_id(id);

                let json = serde_json::to_string(&response).expect("Response serialization failed");
                if let Err(e) = framed.send(json).await {
                    error!("Write error to {addr}: {e}");
                    break;
                }
            }
            Message::Response { .. } => {
                info!("Received response from {addr}: {:?}", msg); //TODO: process response to my request
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