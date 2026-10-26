//! Rust encoder and decoder for slipnet:// URIs.
//! Encodes tunnel configurations into shareable URLs for SlipNet / NoizDNS clients.

use luffy_core::CoreError;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct SlipNetProfile {
    pub tag: String,
    pub transport: String, // dnstt, vaydns, slipstream, naive, stuntls
    pub domain: String,
    pub public_key: Option<String>,
    pub resolver: Option<String>,
    pub transport_protocol: String, // udp, tcp, dot, doh
    pub query_size: Option<u16>,    // 50, 60, 80, 100
    pub utls_fingerprint: Option<String>,
    pub ssh_user: Option<String>,
    pub ssh_password: Option<String>,
    pub socks_user: Option<String>,
    pub socks_password: Option<String>,
}

pub struct SlipNetCodec;

impl SlipNetCodec {
    /// Encodes a SlipNetProfile into a slipnet:// URI
    pub fn encode(profile: &SlipNetProfile) -> String {
        let mut query_params = Vec::new();

        query_params.push(format!("transport={}", urlencoding(&profile.transport)));
        query_params.push(format!("domain={}", urlencoding(&profile.domain)));
        query_params.push(format!("proto={}", urlencoding(&profile.transport_protocol)));

        if let Some(pk) = &profile.public_key {
            query_params.push(format!("pk={}", urlencoding(pk)));
        }
        if let Some(r) = &profile.resolver {
            query_params.push(format!("resolver={}", urlencoding(r)));
        }
        if let Some(qs) = profile.query_size {
            query_params.push(format!("qsize={qs}"));
        }
        if let Some(utls) = &profile.utls_fingerprint {
            query_params.push(format!("utls={}", urlencoding(utls)));
        }
        if let Some(u) = &profile.ssh_user {
            query_params.push(format!("ssh_user={}", urlencoding(u)));
        }
        if let Some(p) = &profile.ssh_password {
            query_params.push(format!("ssh_pass={}", urlencoding(p)));
        }

        format!(
            "slipnet://{}?{}",
            urlencoding(&profile.tag),
            query_params.join("&")
        )
    }

    /// Decodes a slipnet:// URI into a SlipNetProfile
    pub fn decode(uri: &str) -> Result<SlipNetProfile, CoreError> {
        let stripped = uri
            .strip_prefix("slipnet://")
            .ok_or_else(|| CoreError::Validation("Invalid scheme, must start with slipnet://".into()))?;

        let (tag_part, query_part) = match stripped.split_once('?') {
            Some((t, q)) => (urldecoding(t)?, q),
            None => (urldecoding(stripped)?, ""),
        };

        let mut transport = String::new();
        let mut domain = String::new();
        let mut transport_protocol = "udp".to_string();
        let mut public_key = None;
        let mut resolver = None;
        let mut query_size = None;
        let mut utls_fingerprint = None;
        let mut ssh_user = None;
        let mut ssh_password = None;

        for pair in query_part.split('&') {
            if pair.is_empty() {
                continue;
            }
            if let Some((k, v)) = pair.split_once('=') {
                let val = urldecoding(v)?;
                match k {
                    "transport" => transport = val,
                    "domain" => domain = val,
                    "proto" => transport_protocol = val,
                    "pk" => public_key = Some(val),
                    "resolver" => resolver = Some(val),
                    "qsize" => query_size = val.parse::<u16>().ok(),
                    "utls" => utls_fingerprint = Some(val),
                    "ssh_user" => ssh_user = Some(val),
                    "ssh_pass" => ssh_password = Some(val),
                    _ => {}
                }
            }
        }

        if transport.is_empty() || domain.is_empty() {
            return Err(CoreError::Validation(
                "Missing required fields 'transport' or 'domain' in slipnet URI".into(),
            ));
        }

        Ok(SlipNetProfile {
            tag: tag_part,
            transport,
            domain,
            public_key,
            resolver,
            transport_protocol,
            query_size,
            utls_fingerprint,
            ssh_user,
            ssh_password,
            socks_user: None,
            socks_password: None,
        })
    }
}

fn urlencoding(s: &str) -> String {
    let mut encoded = String::new();
    for b in s.bytes() {
        if b.is_ascii_alphanumeric() || b == b'-' || b == b'_' || b == b'.' || b == b'~' {
            encoded.push(b as char);
        } else {
            encoded.push_str(&format!("%{:02X}", b));
        }
    }
    encoded
}

fn urldecoding(s: &str) -> Result<String, CoreError> {
    let mut bytes = Vec::new();
    let mut chars = s.chars().peekable();

    while let Some(c) = chars.next() {
        if c == '%' {
            let h1 = chars.next().ok_or_else(|| CoreError::Validation("Incomplete % hex in URI".into()))?;
            let h2 = chars.next().ok_or_else(|| CoreError::Validation("Incomplete % hex in URI".into()))?;
            let hex_str = format!("{h1}{h2}");
            let byte = u8::from_str_radix(&hex_str, 16)
                .map_err(|_| CoreError::Validation(format!("Invalid hex byte '%{hex_str}'")))?;
            bytes.push(byte);
        } else if c == '+' {
            bytes.push(b' ');
        } else {
            bytes.push(c as u8);
        }
    }

    String::from_utf8(bytes).map_err(|_| CoreError::Validation("Invalid UTF-8 in decoded URI".into()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_slipnet_codec_roundtrip() {
        let profile = SlipNetProfile {
            tag: "VIP Tehran DNSTT".to_string(),
            transport: "dnstt".to_string(),
            domain: "t.example.com".to_string(),
            public_key: Some("0123456789abcdef0123456789abcdef".to_string()),
            resolver: Some("8.8.8.8:53".to_string()),
            transport_protocol: "udp".to_string(),
            query_size: Some(80),
            utls_fingerprint: Some("chrome_120".to_string()),
            ssh_user: Some("luffy_user".to_string()),
            ssh_password: Some("secret123".to_string()),
            socks_user: None,
            socks_password: None,
        };

        let uri = SlipNetCodec::encode(&profile);
        assert!(uri.starts_with("slipnet://"));
        assert!(uri.contains("transport=dnstt"));
        assert!(uri.contains("qsize=80"));

        let decoded = SlipNetCodec::decode(&uri).unwrap();
        assert_eq!(decoded.tag, profile.tag);
        assert_eq!(decoded.domain, profile.domain);
        assert_eq!(decoded.public_key, profile.public_key);
        assert_eq!(decoded.query_size, Some(80));
    }
}
