use crate::utils::Utils;
use hex::encode as hex_encode;
use hmac::{Hmac, Mac};
use sha2::Sha256;

struct Token {
    key: String,
    secret: String,
}

type HmacSha256 = Hmac<Sha256>;

impl Token {
    pub fn new(key: &str, secret: &str) -> Self {
        Token {
            key: key.to_string(),
            secret: secret.to_string(),
        }
    }

    /// Signs the string using the secret.
    pub fn sign(&self, string: &str) -> String {
        let mut mac = HmacSha256::new_from_slice(self.secret.as_bytes())
            .expect("HMAC can take key of any size");
        mac.update(string.as_bytes());
        hex_encode(mac.finalize().into_bytes())
    }

    /// Checks if the string has correct signature.
    pub fn verify(&self, string: &str, signature: &str) -> bool {
        Utils::secure_compare(self.sign(string), signature)
    }
}
