use std::{sync::Arc, fs::File, io::BufReader, path::Path};
use tokio_rustls::{TlsConnector, rustls};
use tokio_rustls::rustls::pki_types::{ServerName, PrivateKeyDer, PrivatePkcs8KeyDer};

fn build_client_tls_config() -> Arc<rustls::ClientConfig> {
    let ca_certs = load_certs(Path::new("certs/dev/ca.crt"));
    let mut roots = rustls::RootCertStore::empty();
    for cert in ca_certs {
        roots.add(cert).unwrap();
    }

    let client_certs = load_certs(Path::new("certs/dev/client.crt"));
    let client_key = load_key(Path::new("certs/dev/client.key"));

    rustls::ClientConfig::builder()
        .with_root_certificates(roots)
        .with_client_auth_cert(client_certs, client_key)
        .unwrap()
        .into()
}