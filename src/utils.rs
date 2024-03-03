use regex::Regex;
use serde_json::to_string;
use std::collections::HashMap;

pub struct Utils;

impl Utils {
    // Associated constants
    const CLIENT_EVENT_PATTERNS: &'static [&'static str] = &["client-*"];
    const PRIVATE_CHANNEL_PATTERNS: &'static [&'static str] =
        &["private-*", "private-encrypted-*", "presence-*"];
    const CACHING_CHANNEL_PATTERNS: &'static [&'static str] = &[
        "cache-*",
        "private-cache-*",
        "private-encrypted-cache-*",
        "presence-cache-*",
    ];

    pub fn data_to_bytes(data: Vec<serde_json::Value>) -> usize {
        data.iter().fold(0, |total_bytes, element| {
            let element_str = to_string(element).unwrap_or_default();
            total_bytes + element_str.as_bytes().len()
        })
    }

    pub fn data_to_kilobytes(data: Vec<serde_json::Value>) -> f64 {
        Self::data_to_bytes(data) as f64 / 1024.0
    }

    pub fn data_to_megabytes(data: Vec<serde_json::Value>) -> f64 {
        Self::data_to_kilobytes(data) / 1024.0
    }

    fn is_match(patterns: &[&str], channel: &str) -> bool {
        patterns.iter().any(|pattern| {
            let regex = Regex::new(&pattern.replace("*", ".*")).unwrap();
            regex.is_match(channel)
        })
    }

    pub fn is_private_channel(channel: &str) -> bool {
        Self::is_match(Self::PRIVATE_CHANNEL_PATTERNS, channel)
    }

    pub fn is_presence_channel(channel: &str) -> bool {
        channel.starts_with("presence-")
    }

    pub fn is_encrypted_private_channel(channel: &str) -> bool {
        channel.starts_with("private-encrypted-")
    }

    pub fn is_caching_channel(channel: &str) -> bool {
        Self::is_match(Self::CACHING_CHANNEL_PATTERNS, channel)
    }

    pub fn is_client_event(event: &str) -> bool {
        Self::is_match(Self::CLIENT_EVENT_PATTERNS, event)
    }

    pub fn restricted_channel_name(name: &str) -> bool {
        !Regex::new(r"^#?[-a-zA-Z0-9_=@,.;]+$")
            .unwrap()
            .is_match(name)
    }
    fn to_ordered_array(map: &HashMap<&str, &str>) -> Vec<String> {
        let mut pairs: Vec<_> = map.iter().collect();
        pairs.sort_by(|a, b| a.0.cmp(b.0));
        pairs.iter().map(|(k, v)| format!("{}={}", k, v)).collect()
    }

    /// Calculates the MD5 hash of the given string.
    fn get_md5(body: &str) -> String {
        format!("{:x}", md5::compute(body))
    }

    /// Securely compares two strings for equality, preventing timing attacks.
    pub(crate) fn secure_compare(a: String, b: &str) -> bool {
        if a.len() != b.len() {
            return false;
        }
        let mut result = 0;
        for (ac, bc) in a.chars().zip(b.chars()) {
            result |= (ac as u32) ^ (bc as u32);
        }
        result == 0
    }

    /// Checks if a channel is encrypted based on its name.
    fn is_encrypted_channel(channel: &str) -> bool {
        channel.starts_with("private-encrypted-")
    }
}
