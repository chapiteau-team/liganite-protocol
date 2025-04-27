use cid::Cid;
use core::str::FromStr;
use url::Url;

pub fn is_cid(cid: &[u8]) -> bool {
    match core::str::from_utf8(cid) {
        Ok(cid) => Cid::from_str(cid).is_ok(),
        Err(_) => false,
    }
}

pub fn is_string(string: &[u8]) -> bool {
    core::str::from_utf8(string).is_ok()
}

pub fn is_non_empty_string(string: &[u8]) -> bool {
    !string.is_empty() && is_string(string)
}

pub fn is_url(url: &[u8]) -> bool {
    match core::str::from_utf8(url) {
        Ok(s) => Url::parse(s).is_ok(),
        Err(_) => false,
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_is_cid() {
        assert!(is_cid(b"QmYwAPJzv5CZsnAztbCxjRMoa6zFzFG8pGzLFZxojtL8MX"));
        assert!(is_cid(b"QmRJzSVrU5kMkXzDCrePyx3TX7gGu8cXsogX5xLyfMuNPG"));
        assert!(is_cid(b"bafybeigdyrzt3whh4p5fy7uj5zd7qvmdtg7okjqcyawh5hj7sgl2xylz4u"));
        assert!(is_cid(b"bafkreihdwdcefgh4dqkjv67uzcmw7ojee6xedzdetojuzjevtenxquvyku"));

        assert!(!is_cid(b"ZfYwAPJzv5CZsnAztbCxjRMoa6zFzFG8pGzLFZxojtL8MX"));
        assert!(!is_cid(b"Qm123"));
        assert!(!is_cid(b"bafybeigd!@#wrongcharacters"));
        assert!(!is_cid(b"bafkreihdwdcefgh4dqkjv67uzcmw7ojee6xedzdetojuzjevtenxquvyk"));
        assert!(!is_cid(b"NotARealCID123"));
    }

    #[test]
    fn test_is_string() {
        assert!(is_string(b"hello"));
        assert!(is_string("こんにちは".as_bytes())); // Japanese
        assert!(is_string("😊🚀".as_bytes())); // Emojis
        assert!(is_string(b"1234567890"));
        assert!(is_string(b"")); // Empty string is valid UTF-8

        assert!(!is_string(&[0xff, 0xfe, 0xfd])); // Clearly invalid
        assert!(!is_string(&[0xc3])); // Incomplete UTF-8 character
        assert!(!is_string(&[0xe2, 0x28, 0xa1])); // Invalid UTF-8
        assert!(!is_string(&[0xf0, 0x28, 0x8c, 0x28])); // Overlong encoding
    }

    #[test]
    fn test_is_non_empty_string() {
        assert!(is_non_empty_string(b"hello"));
        assert!(!is_non_empty_string(b""));
    }

    #[test]
    fn test_is_url() {
        assert!(is_url(b"https://example.com"));
        assert!(is_url(b"http://example.com"));
        assert!(is_url(b"ftp://example.com"));
        assert!(!is_url(b"example.com"));
        assert!(!is_url(b""));
    }
}
