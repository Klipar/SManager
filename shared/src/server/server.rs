use std::{collections::HashMap, sync::Arc, fs::File, io::BufReader};

use tokio::net::TcpListener;
use tokio_rustls::{TlsAcceptor, rustls};
use tokio_util::codec::{Framed, LinesCodec};
use futures::{StreamExt, SinkExt};

use anyhow::Result;
use log::{info, error};

use crate::server::{
    connection_context::ConnectionContext,
    handler_trait::HandlerTrait,
    message::{Message, Status},
};

use tokio_rustls::rustls::pki_types::{PrivateKeyDer, PrivatePkcs8KeyDer};

// ---------------- TLS CONFIG ----------------

fn load_certs(path: &str) -> Vec<rustls::pki_types::CertificateDer<'static>> {
    let certfile = File::open(path).expect(&format!("Failed to load cert: {}",path));
    let mut reader = BufReader::new(certfile);

    rustls_pemfile::certs(&mut reader)
        .map(|c| c.unwrap()) // TODO: improve errors for all casas.
        .collect()
}

fn load_key(path: &str) -> PrivateKeyDer<'static> {
    let keyfile = File::open(path).expect(&format!("Failed to load key: {}",path));
    let mut reader = BufReader::new(keyfile);

    let key: PrivatePkcs8KeyDer<'static> = rustls_pemfile::pkcs8_private_keys(&mut reader)
        .map(|k| k.unwrap())
        .next()
        .expect("No private key found");

    PrivateKeyDer::Pkcs8(key)
}

fn build_tls_config() -> Arc<rustls::ServerConfig> {
    let path_to_certs = match std::env::var("CERTIFICATES_LOCATION") {
        Ok(val) => val,
        Err(_) => {
            log::warn!("CERTIFICATES_LOCATION not set, using default: `certs`");
            "certs".to_string()
        }
    };

    let certs = load_certs(&format!("{}/server.crt", path_to_certs));
    let key = load_key(&format!("{}/server.key", path_to_certs));

    let client_ca = load_certs(&format!("{}/ca.crt", path_to_certs));

    let mut roots = rustls::RootCertStore::empty();
    for cert in client_ca {
        roots.add(cert).unwrap();
    }

    let client_auth = rustls::server::WebPkiClientVerifier::builder(roots.into())
        .build()
        .unwrap();

    let config = rustls::ServerConfig::builder()
        .with_client_cert_verifier(client_auth)
        .with_single_cert(certs, key)
        .unwrap();

    Arc::new(config)
}

// ---------------- SERVER ----------------

pub struct Server {
    pub ip: String,
    pub port: u16,
    pub is_active: bool,
    pub handlers: HashMap<String, Arc<dyn HandlerTrait>>,
}

impl Server {
    pub fn new(ip: String, port: u16) -> Self {
        Self {
            ip,
            port,
            is_active: false,
            handlers: HashMap::new(),
        }
    }

    pub fn add_handler(&mut self, name: &str, handler: Arc<dyn HandlerTrait>) {
        self.handlers.insert(name.to_string(), handler);
    }

    pub async fn start_server(mut self) -> Result<()> {
        let listener = TcpListener::bind(format!("{}:{}", self.ip, self.port)).await?;
        let tls_config = build_tls_config();
        let acceptor = TlsAcceptor::from(tls_config);

        self.is_active = true;

        info!("mTLS Server listening on {}:{}", self.ip, self.port);

        loop {
            let (socket, addr) = listener.accept().await?;
            let acceptor = acceptor.clone();
            let handlers = self.handlers.clone();

            tokio::spawn(async move {
                // ---- TLS HANDSHAKE ----
                let tls_stream = match acceptor.accept(socket).await {
                    Ok(stream) => stream,
                    Err(e) => {
                        error!("TLS error from {}: {}", addr, e);
                        return;
                    }
                };

                info!("TLS connection established: {}", addr);

                let mut framed = Framed::new(
                    tls_stream,
                    LinesCodec::new_with_max_length(65536),
                );

                let mut ctx = ConnectionContext::new(
                    addr.ip().to_string()
                );

                while let Some(result) = framed.next().await {
                    match result {
                        Ok(line) => {
                            if let Ok(msg) = serde_json::from_str::<Message>(&line) {
                                match msg {
                                    Message::Request { id, action, data } => {
                                        if ctx.authenticated || action == "authenticate" {
                                            if let Some(handler) = handlers.get(&action) {
                                                let mut response = handler.handle(data, &mut ctx).await;
                                                response.set_id(id);

                                                let json = serde_json::to_string(&response).unwrap();
                                                if let Err(e) = framed.send(json).await {
                                                    error!("Write error {}: {}", addr, e);
                                                    return;
                                                }
                                            } else {
                                                error!("Unknown request: {}", action);
                                            }
                                        } else {
                                            let response = Message::Response {
                                                id,
                                                status: Status::Error,
                                                data: None,
                                                code: 401,
                                                message: "Unauthorized".to_string()
                                            };

                                            let json = serde_json::to_string(&response).unwrap();
                                            if let Err(e) = framed.send(json).await {
                                                error!("Write error {}: {}", addr, e);
                                                return;
                                            }
                                        }
                                    }
                                    Message::Response { .. } => {
                                        println!("{:?}", msg); //TODO: process responses
                                    }
                                }
                            } else {
                                error!("Failed to parse from {}", addr);
                            }
                        }
                        Err(e) => {
                            error!("Read error {}: {}", addr, e);
                            return;
                        }
                    }
                }

                info!("Disconnected {}", addr);
            });
        }
    }
}