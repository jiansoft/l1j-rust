/// Teleport-related client packet parsers.

use crate::protocol::packet::PacketReader;

/// Parsed C_ENTERPORTAL packet.
pub struct EnterPortal {
    pub x: i32,
    pub y: i32,
}

/// Parse C_ENTERPORTAL - player stepped on a portal tile.
pub fn parse_enter_portal(data: &[u8]) -> EnterPortal {
    let mut r = PacketReader::after_opcode(data);
    let x = r.read_h() as i32;
    let y = r.read_h() as i32;
    EnterPortal { x, y }
}

/// Parsed C_BOOKMARK (add bookmark).
pub struct AddBookmark {
    pub name: String,
}

pub fn parse_add_bookmark(data: &[u8]) -> AddBookmark {
    let mut r = PacketReader::after_opcode(data);
    let name = r.read_s();
    AddBookmark { name }
}

/// Parsed C_BOOKMARKDELETE (delete bookmark).
pub struct DeleteBookmark {
    pub name: String,
}

pub fn parse_delete_bookmark(data: &[u8]) -> DeleteBookmark {
    let mut r = PacketReader::after_opcode(data);
    let name = r.read_s();
    DeleteBookmark { name }
}
