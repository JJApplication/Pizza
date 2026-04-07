use crate::middleware::pre_handler::{PreHandler, PreHandlerResult};
use bytes::Bytes;
use http::Request;
use http::StatusCode;
use http_body_util::Full;

pub struct AuthPreHandler {
    enabled: bool,
    auth_type: String,
    jwt_secret: Option<String>,
    basic_auth_users: Vec<(String, String)>,
}

impl AuthPreHandler {
    pub fn new(
        enabled: bool,
        auth_type: String,
        jwt_secret: Option<String>,
        basic_auth_users: Vec<(String, String)>,
    ) -> Self {
        Self {
            enabled,
            auth_type,
            jwt_secret,
            basic_auth_users,
        }
    }

    fn check_basic_auth(&self, auth_header: &str) -> bool {
        if let Some(credentials) = auth_header.strip_prefix("Basic ") {
            if let Ok(decoded) = decode_base64(credentials) {
                if let Some((user, pass)) = decoded.split_once(':') {
                    return self
                        .basic_auth_users
                        .iter()
                        .any(|(u, p)| u == user && p == pass);
                }
            }
        }
        false
    }

    fn check_jwt(&self, _auth_header: &str) -> bool {
        self.jwt_secret.is_some()
    }
}

impl PreHandler for AuthPreHandler {
    fn name(&self) -> &str {
        "auth"
    }
    fn enabled(&self) -> bool {
        self.enabled
    }

    fn handle(&self, req: &mut Request<Full<Bytes>>) -> Result<(), PreHandlerResult> {
        let auth_header = req
            .headers()
            .get(http::header::AUTHORIZATION)
            .and_then(|v| v.to_str().ok())
            .unwrap_or("");

        if auth_header.is_empty() {
            return Err(PreHandlerResult::reject(
                StatusCode::UNAUTHORIZED,
                "Authentication required",
            ));
        }

        let allowed = match self.auth_type.as_str() {
            "basic" => self.check_basic_auth(auth_header),
            "jwt" => self.check_jwt(auth_header),
            _ => true,
        };

        if !allowed {
            return Err(PreHandlerResult::reject(
                StatusCode::UNAUTHORIZED,
                "Authentication failed",
            ));
        }

        Ok(())
    }
}

fn decode_base64(input: &str) -> Result<String, ()> {
    const ALPHABET: &[u8] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789+/";
    let mut bytes = Vec::new();
    let input = input.trim_end_matches('=');
    let chunks = input.as_bytes().chunks(4);

    for chunk in chunks {
        if chunk.is_empty() {
            continue;
        }
        let mut val: u32 = 0;
        for (i, &b) in chunk.iter().enumerate() {
            if let Some(pos) = ALPHABET.iter().position(|&x| x == b) {
                val |= (pos as u32) << (18 - i * 6);
            }
        }
        bytes.push(((val >> 16) & 0xFF) as u8);
        if chunk.len() > 2 {
            bytes.push(((val >> 8) & 0xFF) as u8);
        }
        if chunk.len() > 3 {
            bytes.push((val & 0xFF) as u8);
        }
    }

    String::from_utf8(bytes).map_err(|_| ())
}
