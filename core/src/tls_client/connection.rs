use std::{sync::Arc, fs::File, io::BufReader, path::{Path, PathBuf}};
use anyhow::Result;
use tokio_rustls::{TlsConnector, rustls};
use tokio_rustls::rustls::pki_types::{ServerName, PrivateKeyDer, PrivatePkcs8KeyDer};
use tokio_util::codec::{Framed, LinesCodec};

// ---- cert helpers (rovnaké ako na serveri) ----

fn load_certs(path: &Path) -> Vec<rustls::pki_types::CertificateDer<'static>> {
    let file = File::open(path).unwrap_or_else(|_| panic!("Failed to load cert: {}", path.display()));
    let mut reader = BufReader::new(file);
    rustls_pemfile::certs(&mut reader)
        .map(|c| c.unwrap())
        .collect()
}

fn load_key(path: &Path) -> PrivateKeyDer<'static> {
    let file = File::open(path).unwrap_or_else(|_| panic!("Failed to load key: {}", path.display()));
    let mut reader = BufReader::new(file);
    let key: PrivatePkcs8KeyDer<'static> = rustls_pemfile::pkcs8_private_keys(&mut reader)
        .map(|k| k.unwrap())
        .next()
        .expect("No private key found");
    PrivateKeyDer::Pkcs8(key)
}

fn resolve_certs_dir() -> PathBuf {
    let configured = std::env::var("CERTIFICATES_LOCATION")
        .unwrap_or_else(|_| {
            log::warn!("CERTIFICATES_LOCATION not set, using default: certs/dev");
            "certs/dev".to_string()
        });

    let p = PathBuf::from(&configured);
    if p.is_absolute() && p.is_dir() {
        return p;
    }

    let manifest = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace = manifest.join("..");

    [
        p.clone(),
        manifest.join(&p),
        workspace.join(&p),
        workspace.join("certs/dev"),
    ]
        .into_iter()
        .find(|p| p.is_dir())
        .unwrap_or_else(|| panic!("Certificates directory not found: {}", configured))
}

// ---- TLS config ----

fn build_client_tls_config() -> Arc<rustls::ClientConfig> {
    let certs_dir = resolve_certs_dir();

    let ca_certs = load_certs(&certs_dir.join("ca.crt"));
    let mut roots = rustls::RootCertStore::empty();
    for cert in ca_certs {
        roots.add(cert).unwrap();
    }

    let client_certs = load_certs(&certs_dir.join("client.crt"));
    let client_key = load_key(&certs_dir.join("client.key"));

    Arc::new(
        rustls::ClientConfig::builder()
            .with_root_certificates(roots)
            .with_client_auth_cert(client_certs, client_key)
            .unwrap(),
    )
}

// ---- public connect ----

pub type AgentFramed = Framed<
    tokio_rustls::client::TlsStream<tokio::net::TcpStream>,
    LinesCodec,
>;

pub async fn connect(server_ip: &str, port: u16, server_cn: &str) -> Result<AgentFramed> {
    let tls_config = build_client_tls_config();
    let connector = TlsConnector::from(tls_config);

    let tcp = tokio::net::TcpStream::connect((server_ip, port)).await?;

    let domain = ServerName::try_from(server_cn.to_string())?;
    let tls_stream = connector.connect(domain, tcp).await?;

    let framed = Framed::new(
        tls_stream,
        LinesCodec::new_with_max_length(65536),
    );

    Ok(framed)
}