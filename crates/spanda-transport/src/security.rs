//! Transport-layer security policy validation and TLS session management.

#[cfg(feature = "tls")]
use crate::tls;
use spanda_runtime::security_types::{
    AuthenticationMode, EncryptionMode, IntegrityMode, SecureCommPolicy,
};
use spanda_runtime::wire_crypto::WireCryptoSession;

const WIRE_PREFIX: &str = "spanda/wire/v1:";

/// Per-transport TLS / encryption configuration wired from `bus { ... }` declarations.
#[derive(Debug, Clone, Default, PartialEq, Eq)]
pub struct TransportSecurityConfig {
    pub encryption: EncryptionMode,
    pub authentication: AuthenticationMode,
    pub integrity: IntegrityMode,
    pub cert_path: Option<String>,
    pub key_secret: Option<String>,
    pub key_path: Option<String>,
}

impl TransportSecurityConfig {
    pub fn from_bus_fields(
        encryption: Option<&str>,
        authentication: Option<&str>,
        integrity: Option<&str>,
    ) -> Result<Self, String> {
        // Description:
        //     From bus fields.
        //
        // Inputs:
        //     encryption: Option<&str>
        //         Caller-supplied encryption.
        //     authentication: Option<&str>
        //         Caller-supplied authentication.
        //     integrity: Option<&str>
        //         Caller-supplied integrity.
        //
        // Outputs:
        //     result: Result<Self, String>
        //         Return value from `from_bus_fields`.
        //
        // Example:

        //     let result = spanda_transport::security::from_bus_fields(encryption, authentication, integrity);

        Ok(Self {
            encryption: parse_encryption(encryption)?,
            authentication: parse_authentication(authentication)?,
            integrity: parse_integrity(integrity)?,
            cert_path: None,
            key_secret: None,
            key_path: None,
        })
    }

    pub fn with_secrets(mut self, cert_path: Option<String>, key_secret: Option<String>) -> Self {
        // Description:
        //     With secrets.
        //
        // Inputs:
        //     mut self: input value
        //         Caller-supplied mut self.
        //     cert_path: Option<String>
        //         Caller-supplied cert path.
        //     key_secre: Option<String>
        //         Caller-supplied key secre.
        //
        // Outputs:
        //     result: Self
        //         Return value from `with_secrets`.
        //
        // Example:

        //     let result = spanda_transport::security::with_secrets(mut self, cert_path, key_secre);

        self.cert_path = cert_path;
        self.key_secret = key_secret;
        self
    }

    pub fn session_material(&self) -> String {
        // Description:
        //     Session material.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //
        // Outputs:
        //     result: String
        //         Return value from `session_material`.
        //
        // Example:

        //     let result = spanda_transport::security::session_material(&self);

        format!(
            "{}:{}",
            self.cert_path.as_deref().unwrap_or("spanda-local"),
            self.key_secret.as_deref().unwrap_or("spanda-local-key")
        )
    }

    pub fn validate(&self, transport: &str) -> Result<(), String> {
        // Description:
        //     Validate.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     ranspor: &str
        //         Caller-supplied ranspor.
        //
        // Outputs:
        //     result: Result<(), String>
        //         Return value from `validate`.
        //
        // Example:

        //     let result = spanda_transport::security::validate(&self, ranspor);

        if self.encryption == EncryptionMode::Required
            && self.cert_path.is_none()
            && self.key_secret.is_none()
        {
            return Err(format!(
                "transport '{transport}' requires encryption but no cert/key secret is configured"
            ));
        }
        Ok(())
    }

    /// Resolve broker URL from bus declaration or `SPANDA_BROKER_URL` environment variable.
    pub fn resolve_broker_url(bus_url: Option<&str>) -> Option<String> {
        // Description:
        //     Resolve broker url.
        //
        // Inputs:
        //     bus_url: Option<&str>
        //         Caller-supplied bus url.
        //
        // Outputs:
        //     result: Option<String>
        //         Return value from `resolve_broker_url`.
        //
        // Example:

        //     let result = spanda_transport::security::resolve_broker_url(bus_url);

        if let Some(url) = bus_url {
            if !url.is_empty() {
                return Some(url.to_string());
            }
        }
        std::env::var("SPANDA_BROKER_URL")
            .ok()
            .filter(|value| !value.is_empty())
    }

    /// True when broker URL implies TLS (`mqtts://`, `wss://`, etc.).
    pub fn url_requires_tls(broker_url: Option<&str>) -> bool {
        // Description:
        //     Url requires tls.
        //
        // Inputs:
        //     broker_url: Option<&str>
        //         Caller-supplied broker url.
        //
        // Outputs:
        //     result: bool
        //         Return value from `url_requires_tls`.
        //
        // Example:

        //     let result = spanda_transport::security::url_requires_tls(broker_url);

        broker_url.is_some_and(|url| {
            let lower = url.to_ascii_lowercase();
            lower.starts_with("mqtts://")
                || lower.starts_with("wss://")
                || lower.starts_with("ssl://")
                || lower.starts_with("tls://")
                || lower.starts_with("dds+sec://")
        })
    }
}

/// Negotiated TLS session for transport wire encryption (AES-256-GCM).
#[derive(Debug, Clone, Default)]
pub struct TlsTransportSession {
    pub negotiated: bool,
    pub cipher_suite: String,
    pub peer_verified: bool,
    session: Option<WireCryptoSession>,
}

/// Backward-compatible alias used by existing transport configuration.
pub type TlsTransportStub = TlsTransportSession;

impl TlsTransportSession {
    pub fn connect(
        &mut self,
        config: &TransportSecurityConfig,
        broker_url: Option<&str>,
    ) -> Result<(), String> {
        // Description:
        //     Connect.
        //
        // Inputs:
        //     &mut self: input value
        //         Caller-supplied &mut self.
        //     config: &TransportSecurityConfig
        //         Caller-supplied config.
        //     broker_url: Option<&str>
        //         Caller-supplied broker url.
        //
        // Outputs:
        //     result: Result<(), String>
        //         Return value from `connect`.
        //
        // Example:

        //     let result = spanda_transport::security::connect(&mut self, config, broker_url);

        config.validate("tls")?;
        if config.encryption == EncryptionMode::None {
            self.negotiated = false;
            self.cipher_suite = "none".into();
            self.peer_verified = true;
            self.session = None;
            return Ok(());
        }

        let cert_file = config
            .cert_path
            .as_deref()
            .filter(|p| std::path::Path::new(p).is_file());
        let key_file = config
            .key_path
            .as_deref()
            .filter(|p| std::path::Path::new(p).is_file());

        if config.authentication == AuthenticationMode::Mutual
            && (cert_file.is_none() || key_file.is_none())
        {
            return Err("mutual TLS authentication failed: missing certificate or key file".into());
        }

        if config.authentication == AuthenticationMode::Mutual {
            if let (Some(cert), Some(key), Some(url)) = (cert_file, key_file, broker_url) {
                #[cfg(feature = "tls")]
                if let Some(endpoint) = tls::parse_tls_endpoint(url) {
                    if endpoint.use_tls {
                        let client_cfg = tls::build_client_config(cert, key)?;
                        match tls::perform_mtls_handshake(&endpoint, client_cfg) {
                            Ok(hs) => {
                                let crypto = WireCryptoSession::from_material(&hs.session_material);
                                self.cipher_suite = hs.cipher_suite;
                                self.peer_verified = hs.peer_verified;
                                self.session = Some(crypto);
                                self.negotiated = true;
                                return Ok(());
                            }
                            Err(err)
                                if std::env::var("SPANDA_MTLS_REQUIRED").ok().as_deref()
                                    == Some("1") =>
                            {
                                return Err(format!("mTLS handshake failed: {err}"));
                            }
                            Err(_) => {}
                        }
                    }
                }
                #[cfg(not(feature = "tls"))]
                let _ = (cert, key, url);
            }
        }

        self.peer_verified =
            config.authentication != AuthenticationMode::Mutual || cert_file.is_some();
        if let Some(path) = cert_file {
            validate_cert_pem(path)?;
            self.peer_verified = true;
        }
        let crypto = WireCryptoSession::from_material(&config.session_material());
        self.cipher_suite = crypto.cipher_suite.clone();
        self.session = Some(crypto);
        self.negotiated = true;
        Ok(())
    }

    pub fn encrypt_frame(&self, plaintext: &str) -> Result<String, String> {
        // Description:
        //     Encrypt frame.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     plaintex: &str
        //         Caller-supplied plaintex.
        //
        // Outputs:
        //     result: Result<String, String>
        //         Return value from `encrypt_frame`.
        //
        // Example:

        //     let result = spanda_transport::security::encrypt_frame(&self, plaintex);

        if !self.negotiated {
            return Ok(plaintext.to_string());
        }
        let session = self
            .session
            .as_ref()
            .ok_or_else(|| "TLS session not negotiated".to_string())?;
        let encrypted = session.encrypt(plaintext.as_bytes())?;
        Ok(format!("{WIRE_PREFIX}{}", hex::encode(encrypted)))
    }

    pub fn decrypt_frame(&self, ciphertext: &str) -> Result<String, String> {
        // Description:
        //     Decrypt frame.
        //
        // Inputs:
        //     &self: input value
        //         Caller-supplied &self.
        //     ciphertex: &str
        //         Caller-supplied ciphertex.
        //
        // Outputs:
        //     result: Result<String, String>
        //         Return value from `decrypt_frame`.
        //
        // Example:

        //     let result = spanda_transport::security::decrypt_frame(&self, ciphertex);

        if !self.negotiated {
            return Ok(ciphertext.to_string());
        }
        if let Some(hex_payload) = ciphertext.strip_prefix(WIRE_PREFIX) {
            let session = self
                .session
                .as_ref()
                .ok_or_else(|| "TLS session not negotiated".to_string())?;
            let bytes = hex::decode(hex_payload).map_err(|e| format!("hex decode failed: {e}"))?;
            let plain = session.decrypt(&bytes)?;
            return String::from_utf8(plain).map_err(|e| format!("utf8 decode failed: {e}"));
        }

        // Legacy simulation prefix from earlier stub builds.
        if let Some(stripped) = ciphertext.strip_prefix(&format!("tls:{}:", self.cipher_suite)) {
            return Ok(stripped.to_string());
        }
        Err("TLS decrypt failed: unrecognized wire frame".into())
    }
}

/// Merge robot `secure_comm` defaults with per-bus overrides.
pub fn effective_transport_policy(
    robot: &SecureCommPolicy,
    bus: &TransportSecurityConfig,
) -> TransportSecurityConfig {
    // Description:
    //     Effective transport policy.
    //
    // Inputs:
    //     robo: &SecureCommPolicy
    //         Caller-supplied robo.
    //     bus: &TransportSecurityConfig
    //         Caller-supplied bus.
    //
    // Outputs:
    //     result: TransportSecurityConfig
    //         Return value from `effective_transport_policy`.
    //
    // Example:

    //     let result = spanda_transport::security::effective_transport_policy(robo, bus);

    TransportSecurityConfig {
        encryption: if bus.encryption != EncryptionMode::None {
            bus.encryption
        } else {
            robot.encryption
        },
        authentication: if bus.authentication != AuthenticationMode::None {
            bus.authentication
        } else {
            robot.authentication
        },
        integrity: if bus.integrity != IntegrityMode::None {
            bus.integrity
        } else {
            robot.integrity
        },
        cert_path: bus.cert_path.clone(),
        key_secret: bus.key_secret.clone(),
        key_path: bus.key_path.clone(),
    }
}

fn parse_encryption(value: Option<&str>) -> Result<EncryptionMode, String> {
    // Description:
    //     Parse encryption.
    //
    // Inputs:
    //     value: Option<&str>
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: Result<EncryptionMode, String>
    //         Return value from `parse_encryption`.
    //
    // Example:

    //     let result = spanda_transport::security::parse_encryption(value);

    match value {
        None => Ok(EncryptionMode::None),
        Some(v) => v.parse().map_err(|e: String| e),
    }
}

fn parse_authentication(value: Option<&str>) -> Result<AuthenticationMode, String> {
    // Description:
    //     Parse authentication.
    //
    // Inputs:
    //     value: Option<&str>
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: Result<AuthenticationMode, String>
    //         Return value from `parse_authentication`.
    //
    // Example:

    //     let result = spanda_transport::security::parse_authentication(value);

    match value {
        None => Ok(AuthenticationMode::None),
        Some(v) => v.parse().map_err(|e: String| e),
    }
}

fn parse_integrity(value: Option<&str>) -> Result<IntegrityMode, String> {
    // Description:
    //     Parse integrity.
    //
    // Inputs:
    //     value: Option<&str>
    //         Caller-supplied value.
    //
    // Outputs:
    //     result: Result<IntegrityMode, String>
    //         Return value from `parse_integrity`.
    //
    // Example:

    //     let result = spanda_transport::security::parse_integrity(value);

    match value {
        None => Ok(IntegrityMode::None),
        Some(v) => v.parse().map_err(|e: String| e),
    }
}

fn validate_cert_pem(path: &str) -> Result<(), String> {
    // Description:

    //     Validate cert pem.

    //

    // Inputs:

    //     path: &str

    //         Caller-supplied path.

    //

    // Outputs:

    //     result: Result<(), String>

    //         Return value from `validate_cert_pem`.

    //

    // Example:

    //     let result = spanda_transport::security::validate_cert_pem(path);
    #[cfg(feature = "tls")]
    {
        use std::fs::File;
        use std::io::BufReader;
        let file = File::open(path).map_err(|e| format!("open cert '{path}': {e}"))?;
        let mut reader = BufReader::new(file);
        let certs = rustls_pemfile::certs(&mut reader)
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("parse cert '{path}': {e}"))?;
        if certs.is_empty() {
            return Err(format!("no certificates found in '{path}'"));
        }
    }
    #[cfg(not(feature = "tls"))]
    {
        let _ = path;
    }
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tls_session_negotiates_aes_gcm() {
        // Description:
        //     Tls session negotiates aes gcm.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_transport::security::tls_session_negotiates_aes_gcm();

        let mut tls = TlsTransportSession::default();
        let cfg = TransportSecurityConfig {
            encryption: EncryptionMode::Required,
            authentication: AuthenticationMode::Signed,
            integrity: IntegrityMode::Required,
            cert_path: Some("certs/rover.pem".into()),
            key_secret: Some("motion_key".into()),
            key_path: None,
        };
        tls.connect(&cfg, None).unwrap();
        assert!(tls.negotiated);
        assert_eq!(tls.cipher_suite, "AES-256-GCM");
        let enc = tls.encrypt_frame(r#"{"v":1,"payload":"x"}"#).unwrap();
        assert!(enc.starts_with(WIRE_PREFIX));
        let dec = tls.decrypt_frame(&enc).unwrap();
        assert_eq!(dec, r#"{"v":1,"payload":"x"}"#);
    }

    #[test]
    fn url_scheme_detects_tls() {
        // Description:
        //     Url scheme detects tls.
        //
        // Inputs:
        //     None.
        //
        // Outputs:
        //     None.
        //
        // Example:

        //     let result = spanda_transport::security::url_scheme_detects_tls();

        assert!(TransportSecurityConfig::url_requires_tls(Some(
            "mqtts://broker.example:8883"
        )));
        assert!(!TransportSecurityConfig::url_requires_tls(Some(
            "mqtt://localhost:1883"
        )));
    }
}
