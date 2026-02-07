/// Big5/MS950 string encoding for the Taiwan 3.80c client.
///
/// The L1J client uses MS950 (Big5 superset) for string encoding.
/// All strings in packets must be encoded to/from Big5 bytes
/// with null termination.

use encoding_rs::BIG5;

/// Encode a Rust UTF-8 string to Big5 bytes for sending to the client.
///
/// Returns the bytes WITHOUT the null terminator (caller adds it).
/// Falls back gracefully for characters that can't be encoded in Big5.
pub fn encode_big5(text: &str) -> Vec<u8> {
    let (encoded, _encoding, _had_errors) = BIG5.encode(text);
    encoded.into_owned()
}

/// Decode Big5 bytes from the client into a Rust UTF-8 String.
///
/// Input should NOT include the null terminator.
pub fn decode_big5(bytes: &[u8]) -> String {
    let (decoded, _encoding, _had_errors) = BIG5.decode(bytes);
    decoded.into_owned()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ascii_roundtrip() {
        let original = "Hello World";
        let encoded = encode_big5(original);
        let decoded = decode_big5(&encoded);
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_chinese_roundtrip() {
        let original = "天堂伺服器";
        let encoded = encode_big5(original);
        let decoded = decode_big5(&encoded);
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_mixed_text() {
        let original = "+7 裁決之劍";
        let encoded = encode_big5(original);
        let decoded = decode_big5(&encoded);
        assert_eq!(decoded, original);
    }

    #[test]
    fn test_big5_encoded_length() {
        // Each Chinese character = 2 bytes in Big5
        let text = "天堂";
        let encoded = encode_big5(text);
        assert_eq!(encoded.len(), 4); // 2 chars * 2 bytes
    }
}
