use crate::protocol::packet::PacketReader;

/// Parsed C_LOGINTOSERVER packet (character selection).
pub struct LoginToServer {
    pub char_name: String,
}

/// Parse the C_LOGINTOSERVER packet.
///
/// The client sends the character name to select which character to play.
pub fn parse_login_to_server(data: &[u8]) -> LoginToServer {
    let mut r = PacketReader::after_opcode(data);
    let char_name = r.read_s();
    LoginToServer { char_name }
}
