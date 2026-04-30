use tokio_rustls::rustls::pki_types::{PrivateKeyDer, PrivatePkcs8KeyDer, CertificateDer};
use std::{sync::Arc, fs::File, io::BufReader, path::{Path, PathBuf}};
use tokio_rustls::rustls;

use anyhow::{Context, Result};
use log::{warn};


fn load_certs(path: &Path) -> Result<Vec<CertificateDer<'static>>> {
    let certfile = File::open(path)
        .with_context(|| format!("Failed to open cert file: {}", path.display()))?;
    let mut reader = BufReader::new(certfile);

    rustls_pemfile::certs(&mut reader)
        .map(|c| c.with_context(|| format!("Failed to parse cert from {}", path.display())))
        .collect()
}

fn load_key(path: &Path) -> Result<PrivateKeyDer<'static>> {
    let keyfile = File::open(path)
        .with_context(|| format!("Failed to open key file: {}", path.display()))?;
    let mut reader = BufReader::new(keyfile);

    let key: PrivatePkcs8KeyDer<'static> = rustls_pemfile::pkcs8_private_keys(&mut reader)
        .map(|k| k.with_context(|| "Failed to parse private key"))
        .collect::<Result<Vec<_>>>()?
        .into_iter()
        .next()
        .context("No private key found in file")?;

    Ok(PrivateKeyDer::Pkcs8(key))
}

pub fn build_tls_config() -> Result<Arc<rustls::ServerConfig>> {
    let configured_dir = std::env::var("CERTIFICATES_LOCATION").unwrap_or_else(|_| {
        warn!("CERTIFICATES_LOCATION not set, using default: certs/dev");
        "certs/dev".to_string()
    });

    let certs_dir = resolve_certificates_dir(&configured_dir)
        .with_context(|| format!(
            "Certificates directory not found. Checked CERTIFICATES_LOCATION='{}'",
            configured_dir
        ))?;

    let certs = load_certs(&certs_dir.join("server.crt"))?;
    let key = load_key(&certs_dir.join("server.key"))?;
    let client_ca = load_certs(&certs_dir.join("ca.crt"))?;

    let mut roots = rustls::RootCertStore::empty();
    for cert in client_ca {
        roots.add(cert).context("Failed to add CA cert to root store")?;
    }

    let client_auth = rustls::server::WebPkiClientVerifier::builder(roots.into())
        .build()
        .context("Failed to build client verifier")?;

    let config = rustls::ServerConfig::builder()
        .with_client_cert_verifier(client_auth)
        .with_single_cert(certs, key)
        .context("Failed to build TLS server config")?;

    Ok(Arc::new(config))
}

fn resolve_certificates_dir(configured: &str) -> Option<PathBuf> {
    let configured_path = PathBuf::from(configured);

    if configured_path.is_absolute() && configured_path.is_dir() {
        return Some(configured_path);
    }

    let manifest_dir = PathBuf::from(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = manifest_dir.join("..");

    [
        configured_path.clone(),
        manifest_dir.join(&configured_path),
        workspace_root.join(&configured_path),
        workspace_root.join("certs/dev"),
    ]
    .into_iter()
    .find(|p| p.is_dir())
}