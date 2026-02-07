/// Packet frame codec for Lineage 1 protocol.
///
/// Frame format: [2-byte LE length][payload]
/// The length field includes itself (length = payload.len() + 2).
///
/// This module is provided for future use with tokio_util::codec::Framed.
/// Currently the session handler does direct reads/writes.

/// Encode a packet payload into a framed byte vector.
///
/// Returns: [length_lo, length_hi, ...payload]
pub fn encode_frame(payload: &[u8]) -> Vec<u8> {
    let length = (payload.len() + 2) as u16;
    let mut frame = Vec::with_capacity(2 + payload.len());
    frame.push((length & 0xFF) as u8);
    frame.push((length >> 8) as u8);
    frame.extend_from_slice(payload);
    frame
}

/// Calculate data length from the 2-byte LE length header.
///
/// Returns None if the length is invalid (< 2 or > 65535).
pub fn decode_length(lo: u8, hi: u8) -> Option<usize> {
    let length = (hi as u16) << 8 | (lo as u16);
    if length < 2 {
        return None;
    }
    let data_len = (length - 2) as usize;
    if data_len > 65533 {
        return None;
    }
    Some(data_len)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_frame() {
        let payload = vec![0x96, 0x01, 0x02, 0x03]; // opcode 150 + 3 bytes
        let frame = encode_frame(&payload);
        assert_eq!(frame[0], 6); // length = 4 + 2 = 6
        assert_eq!(frame[1], 0);
        assert_eq!(&frame[2..], &payload);
    }

    #[test]
    fn test_decode_length_valid() {
        assert_eq!(decode_length(6, 0), Some(4));
        assert_eq!(decode_length(18, 0), Some(16));
        assert_eq!(decode_length(0x00, 0x01), Some(254)); // 256 - 2
    }

    #[test]
    fn test_decode_length_invalid() {
        assert_eq!(decode_length(0, 0), None); // length 0
        assert_eq!(decode_length(1, 0), None); // length 1 (< 2)
    }
}
