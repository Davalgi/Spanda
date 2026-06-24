//! rustls mTLS handshake helpers for live transport endpoints.

use rustls::pki_types::{CertificateDer, PrivateKeyDer, ServerName};
use rustls::{ClientConfig, ClientConnection, RootCertStore, StreamOwned};
use sha2::{Digest, Sha256};
use std::fs::File;
use std::io::{BufReader, Write};
use std::net::{TcpStream, ToSocketAddrs};
use std::sync::Arc;
use std::time::Duration;

/// Parsed host/port endpoint for TLS transports.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct TlsEndpoint {
    pub host: String,
    pub port: u16,
    pub use_tls: bool,
}

/// Result of a completed mTLS handshake against a live broker.
#[derive(Debug, Clone)]
pub struct MtlsHandshakeResult {
    pub cipher_suite: String,
    pub session_material: String,
    pub peer_verified: bool,
}

/// Parse broker URLs into TLS-capable endpoints.
pub fn parse_tls_endpoint(url: &str) -> Option<TlsEndpoint> {
    // Description:
    //     Parse tls endpoint.
    //
    // Inputs:
    //     url: &str
    //         Caller-supplied url.
    //
    // Outputs:
    //     result: Option<TlsEndpoint>
    //         Return value from `parse_tls_endpoint`.
    //
    // Example:

    //     let result = spanda_transport::tls::parse_tls_endpoint(rl);

    let lower = url.to_ascii_lowercase();
    let (use_tls, stripped, default_port) = if let Some(rest) = lower.strip_prefix("mqtts://") {
        (true, rest, 8883_u16)
    } else if let Some(rest) = lower.strip_prefix("mqtt://") {
        (false, rest, 1883)
    } else if let Some(rest) = lower.strip_prefix("wss://") {
        (true, rest, 443)
    } else if let Some(rest) = lower.strip_prefix("ws://") {
        (false, rest, 80)
    } else if let Some(rest) = lower.strip_prefix("dds+sec://") {
        (true, rest, 7400)
    } else if let Some(rest) = lower.strip_prefix("dds://") {
        (false, rest, 7400)
    } else {
        return None;
    };
    let (host, port) = stripped
        .split_once(':')
        .map(|(h, p)| (h.to_string(), p.parse().unwrap_or(default_port)))
        .unwrap_or((stripped.to_string(), default_port));
    Some(TlsEndpoint {
        host,
        port,
        use_tls,
    })
}

fn load_pem_certs(path: &str) -> Result<Vec<CertificateDer<'static>>, String> {
    // Description:
    //     Load pem certs.
    //
    // Inputs:
    //     path: &str
    //         Caller-supplied path.
    //
    // Outputs:
    //     result: Result<Vec<CertificateDer<'static>>, String>
    //         Return value from `load_pem_certs`.
    //
    // Example:

    //     let result = spanda_transport::tls::load_pem_certs(path);

    let file = File::open(path).map_err(|e| format!("open cert '{path}': {e}"))?;
    let mut reader = BufReader::new(file);
    rustls_pemfile::certs(&mut reader)
        .collect::<Result<Vec<_>, _>>()
        .map_err(|e| format!("parse cert '{path}': {e}"))
}

fn load_pem_key(path: &str) -> Result<PrivateKeyDer<'static>, String> {
    // Description:
    //     Load pem key.
    //
    // Inputs:
    //     path: &str
    //         Caller-supplied path.
    //
    // Outputs:
    //     result: Result<PrivateKeyDer<'static>, String>
    //         Return value from `load_pem_key`.
    //
    // Example:

    //     let result = spanda_transport::tls::load_pem_key(path);

    let file = File::open(path).map_err(|e| format!("open key '{path}': {e}"))?;
    let mut reader = BufReader::new(file);
    rustls_pemfile::private_key(&mut reader)
        .map_err(|e| format!("parse key '{path}': {e}"))?
        .ok_or_else(|| format!("no private key found in '{path}'"))
}

/// Build a rustls client configuration with optional client authentication.
pub fn build_client_config(cert_path: &str, key_path: &str) -> Result<Arc<ClientConfig>, String> {
    // Description:
    //     Build client config.
    //
    // Inputs:
    //     cert_path: &str
    //         Caller-supplied cert path.
    //     key_path: &str
    //         Caller-supplied key path.
    //
    // Outputs:
    //     result: Result<Arc<ClientConfig>, String>
    //         Return value from `build_client_config`.
    //
    // Example:

    //     let result = spanda_transport::tls::build_client_config(cert_path, key_path);

    let certs = load_pem_certs(cert_path)?;
    let key = load_pem_key(key_path)?;
    let mut roots = RootCertStore::empty();
    roots.extend(webpki_roots::TLS_SERVER_ROOTS.iter().cloned());
    let config = ClientConfig::builder()
        .with_root_certificates(roots)
        .with_client_auth_cert(certs, key)
        .map_err(|e| format!("client auth config: {e}"))?;
    Ok(Arc::new(config))
}

fn complete_handshake(conn: &mut ClientConnection, tcp: &mut TcpStream) -> Result<(), String> {
    // Description:

    //     Complete handshake.

    //

    // Inputs:

    //     conn: &mut ClientConnection

    //         Caller-supplied conn.

    //     cp: &mut TcpStream

    //         Caller-supplied cp.

    //

    // Outputs:

    //     result: Result<(), String>

    //         Return value from `complete_handshake`.

    //

    // Example:

    //     let result = spanda_transport::tls::complete_handshake(conn, cp);
    while conn.is_handshaking() {
        if conn.wants_write() {
            conn.write_tls(tcp).map_err(|e| format!("tls write: {e}"))?;
        }
        if conn.wants_read() && conn.read_tls(tcp).is_err() {
            break;
        }
        conn.process_new_packets()
            .map_err(|e| format!("tls process: {e}"))?;
    }
    if conn.is_handshaking() {
        return Err("TLS handshake incomplete".into());
    }
    Ok(())
}

fn session_material_from_peer(conn: &ClientConnection, endpoint: &TlsEndpoint) -> String {
    // Description:

    //     Session material from peer.

    //

    // Inputs:

    //     conn: &ClientConnection

    //         Caller-supplied conn.

    //     endpoin: &TlsEndpoint

    //         Caller-supplied endpoin.

    //

    // Outputs:

    //     result: String

    //         Return value from `session_material_from_peer`.

    //

    // Example:

    //     let result = spanda_transport::tls::session_material_from_peer(conn, endpoin);
    let peer = conn
        .peer_certificates()
        .and_then(|certs| certs.first())
        .map(|cert| {
            let mut hasher = Sha256::new();
            hasher.update(cert.as_ref());
            hex::encode(hasher.finalize())
        })
        .unwrap_or_else(|| "no-peer-cert".into());
    format!("mtls:{}:{}:{}", endpoint.host, endpoint.port, peer)
}

/// Perform mTLS handshake against a live broker endpoint.
pub fn perform_mtls_handshake(
    endpoint: &TlsEndpoint,
    client_config: Arc<ClientConfig>,
) -> Result<MtlsHandshakeResult, String> {
    // Description:
    //     Perform mtls handshake.
    //
    // Inputs:
    //     endpoin: &TlsEndpoint
    //         Caller-supplied endpoin.
    //     client_config: Arc<ClientConfig>
    //         Caller-supplied client config.
    //
    // Outputs:
    //     result: Result<MtlsHandshakeResult, String>
    //         Return value from `perform_mtls_handshake`.
    //
    // Example:

    //     let result = spanda_transport::tls::perform_mtls_handshake(endpoin, client_config);

    let addr = format!("{}:{}", endpoint.host, endpoint.port);
    let socket_addr = addr
        .to_socket_addrs()
        .map_err(|e| format!("resolve {addr}: {e}"))?
        .next()
        .ok_or_else(|| format!("no address for {addr}"))?;
    let mut tcp = TcpStream::connect_timeout(&socket_addr, Duration::from_secs(3))
        .map_err(|e| format!("tcp connect {addr}: {e}"))?;
    tcp.set_read_timeout(Some(Duration::from_secs(3)))
        .map_err(|e| format!("read timeout: {e}"))?;
    tcp.set_write_timeout(Some(Duration::from_secs(3)))
        .map_err(|e| format!("write timeout: {e}"))?;

    let server_name = ServerName::try_from(endpoint.host.as_str())
        .map_err(|e| format!("invalid server name: {e}"))?
        .to_owned();
    let mut conn = ClientConnection::new(client_config, server_name)
        .map_err(|e| format!("client connection: {e}"))?;
    complete_handshake(&mut conn, &mut tcp)?;

    let cipher_suite = conn
        .negotiated_cipher_suite()
        .map(|suite| format!("{:?}", suite.suite()))
        .unwrap_or_else(|| "TLS".into());
    let session_material = session_material_from_peer(&conn, endpoint);

    // Drain any alert/write buffers before returning.
    let mut stream = StreamOwned::new(conn, tcp);
    let _ = stream.flush();

    Ok(MtlsHandshakeResult {
        cipher_suite,
        session_material,
        peer_verified: true,
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parses_mqtts_endpoint() {
        // Description:
        //     Parses mqtts endpoint.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_transport::tls::parses_mqtts_endpoint();

        let ep = parse_tls_endpoint("mqtts://broker.example:8883").unwrap();
        assert!(ep.use_tls);
        assert_eq!(ep.host, "broker.example");
        assert_eq!(ep.port, 8883);
    }

    #[test]
    fn parses_wss_endpoint() {
        // Description:
        //     Parses wss endpoint.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_transport::tls::parses_wss_endpoint();

        let ep = parse_tls_endpoint("wss://hub.local:9090").unwrap();
        assert!(ep.use_tls);
        assert_eq!(ep.port, 9090);
    }
}
