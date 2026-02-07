use crate::protocol::packet::PacketReader;

/// Chat types matching the L1J client protocol.
pub const CHAT_NORMAL: u8 = 0;
pub const CHAT_SHOUT: u8 = 2;
pub const CHAT_WORLD: u8 = 3;
pub const CHAT_CLAN: u8 = 4;
pub const CHAT_PARTY: u8 = 11;
pub const CHAT_TRADE: u8 = 12;
pub const CHAT_ALLIANCE: u8 = 13;
pub const CHAT_CHATPARTY: u8 = 14;

/// Parsed C_CHAT packet.
pub struct ChatMessage {
    pub chat_type: u8,
    pub text: String,
}

pub fn parse_chat(data: &[u8]) -> ChatMessage {
    let mut r = PacketReader::after_opcode(data);
    let chat_type = r.read_c();
    let text = r.read_s();
    ChatMessage { chat_type, text }
}
