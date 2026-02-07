/// XOR-based packet cipher for Lineage 1 (3.80c TW).
///
/// Ported 1:1 from Java `Cipher.java`.
/// Each client session gets its own Cipher instance with independent
/// encrypt (eb) and decrypt (db) key states.

const C1: u32 = 0x9c30d539;
const C2: u32 = 0x930fd7e2;
const C3: u32 = 0x7c72e993;
const C4: u32 = 0x287effc3;

pub struct Cipher {
    /// Encrypt key (8 bytes, updated after each encrypt call)
    eb: [u8; 8],
    /// Decrypt key (8 bytes, updated after each decrypt call)
    db: [u8; 8],
}

impl Cipher {
    /// Create a new Cipher from the random handshake key.
    ///
    /// The key is the same random u32 sent to the client during handshake.
    /// Both server and client derive identical initial eb/db keys from it.
    pub fn new(key: u32) -> Self {
        let mut keys = [key ^ C1, C2];
        keys[0] = keys[0].rotate_left(0x13);
        keys[1] ^= keys[0] ^ C3;

        let mut eb = [0u8; 8];
        let mut db = [0u8; 8];

        for i in 0..2usize {
            for j in 0..4usize {
                let b = (keys[i] >> (j * 8)) as u8;
                eb[i * 4 + j] = b;
                db[i * 4 + j] = b;
            }
        }

        Cipher { eb, db }
    }

    /// Encrypt data in-place (server → client).
    ///
    /// Data must be at least 4 bytes (L1J packets are always padded to 4-byte alignment).
    /// After encryption, the encrypt key (eb) is updated for the next packet.
    pub fn encrypt(&mut self, data: &mut [u8]) {
        debug_assert!(
            data.len() >= 4,
            "encrypt: data must be >= 4 bytes, got {}",
            data.len()
        );

        // Save original first 4 bytes for key update
        let mut tb = [0u8; 4];
        tb.copy_from_slice(&data[..4]);

        // XOR chain forward
        data[0] ^= self.eb[0];
        for i in 1..data.len() {
            data[i] ^= data[i - 1] ^ self.eb[i & 7];
        }

        // Special first-4-byte mixing
        data[3] ^= self.eb[2];
        data[2] ^= self.eb[3] ^ data[3];
        data[1] ^= self.eb[4] ^ data[2];
        data[0] ^= self.eb[5] ^ data[1];

        // Update encrypt key using original plaintext
        Self::update_key(&mut self.eb, &tb);
    }

    /// Decrypt data in-place (client → server).
    ///
    /// Data must be at least 4 bytes.
    /// After decryption, the decrypt key (db) is updated for the next packet.
    pub fn decrypt(&mut self, data: &mut [u8]) {
        debug_assert!(
            data.len() >= 4,
            "decrypt: data must be >= 4 bytes, got {}",
            data.len()
        );

        // Reverse the special first-4-byte mixing
        data[0] ^= self.db[5] ^ data[1];
        data[1] ^= self.db[4] ^ data[2];
        data[2] ^= self.db[3] ^ data[3];
        data[3] ^= self.db[2];

        // XOR chain backward
        for i in (1..data.len()).rev() {
            data[i] ^= data[i - 1] ^ self.db[i & 7];
        }
        data[0] ^= self.db[0];

        // Update decrypt key using decrypted plaintext
        let mut ref_bytes = [0u8; 4];
        ref_bytes.copy_from_slice(&data[..4]);
        Self::update_key(&mut self.db, &ref_bytes);
    }

    /// Update a key array using reference bytes.
    ///
    /// XOR first 4 bytes with reference, then apply C4 constant addition
    /// to the upper 4 bytes.
    fn update_key(key: &mut [u8; 8], reference: &[u8; 4]) {
        for i in 0..4 {
            key[i] ^= reference[i];
        }

        let int32 = ((key[7] as u32) << 24)
            | ((key[6] as u32) << 16)
            | ((key[5] as u32) << 8)
            | (key[4] as u32);
        let int32 = int32.wrapping_add(C4);

        for i in 0..4 {
            key[i + 4] = (int32 >> (i * 8)) as u8;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cipher_init_deterministic() {
        let c1 = Cipher::new(0x12345678);
        let c2 = Cipher::new(0x12345678);
        assert_eq!(c1.eb, c2.eb);
        assert_eq!(c1.db, c2.db);
    }

    #[test]
    fn test_encrypt_decrypt_roundtrip() {
        // Server encrypts, client decrypts using same key
        let mut server_cipher = Cipher::new(0xDEADBEEF);
        let mut client_cipher = Cipher::new(0xDEADBEEF);

        let original = b"Hello, Lineage!".to_vec();

        // Pad to 4-byte alignment (like Java ServerBasePacket.getBytes())
        let mut data = original.clone();
        while data.len() % 4 != 0 {
            data.push(0);
        }

        let plaintext_copy = data.clone();

        // Server encrypts
        server_cipher.encrypt(&mut data);
        assert_ne!(data, plaintext_copy, "Encrypted data should differ from plaintext");

        // Client decrypts (using db key, same initial state)
        client_cipher.decrypt(&mut data);
        assert_eq!(data, plaintext_copy, "Decrypted data should match original");
    }

    #[test]
    fn test_multiple_packets_stay_in_sync() {
        let mut server = Cipher::new(0xCAFEBABE);
        let mut client = Cipher::new(0xCAFEBABE);

        for i in 0..10 {
            let mut data = vec![i as u8; 8 + (i * 4)]; // Varying sizes, 4-byte aligned
            let original = data.clone();

            server.encrypt(&mut data);
            client.decrypt(&mut data);

            assert_eq!(data, original, "Packet {} roundtrip failed", i);
        }
    }

    #[test]
    fn test_minimum_4_byte_packet() {
        let mut enc = Cipher::new(0x11111111);
        let mut dec = Cipher::new(0x11111111);

        let mut data = vec![0xAA, 0xBB, 0xCC, 0xDD];
        let original = data.clone();

        enc.encrypt(&mut data);
        dec.decrypt(&mut data);
        assert_eq!(data, original);
    }

    #[test]
    fn test_key_diverges_without_sync() {
        let mut enc = Cipher::new(0x99999999);
        let mut dec = Cipher::new(0x99999999);

        // Encrypt two packets on server side
        let mut pkt1 = vec![1, 2, 3, 4, 5, 6, 7, 8];
        let mut pkt2 = vec![10, 20, 30, 40];

        enc.encrypt(&mut pkt1);
        enc.encrypt(&mut pkt2);

        // Client only decrypts first packet
        dec.decrypt(&mut pkt1);
        // If we try to decrypt pkt2 now, it should work because keys are in sync
        let pkt2_orig = vec![10, 20, 30, 40];
        dec.decrypt(&mut pkt2);
        assert_eq!(pkt2, pkt2_orig, "Sequential decrypt should stay in sync");
    }
}
