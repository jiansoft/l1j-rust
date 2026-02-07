use crate::protocol::packet::PacketReader;

/// Parsed C_MOVECHAR packet.
pub struct MoveChar {
    pub heading: i32,
}

/// Parse C_MOVECHAR.
/// Taiwan client (3.80c): heading is XOR'd with 0x49.
pub fn parse_move_char(data: &[u8]) -> MoveChar {
    let mut r = PacketReader::after_opcode(data);
    let _locx = r.read_h();  // client's reported X (we use server-side pos)
    let _locy = r.read_h();  // client's reported Y
    let raw_heading = r.read_c() as i32;
    let heading = (raw_heading ^ 0x49) & 7; // Taiwan client XOR decode

    MoveChar { heading }
}

/// Parsed C_CHANGEHEADING packet.
pub struct ChangeHeading {
    pub heading: i32,
}

pub fn parse_change_heading(data: &[u8]) -> ChangeHeading {
    let mut r = PacketReader::after_opcode(data);
    let heading = r.read_c() as i32;
    ChangeHeading { heading: heading & 7 }
}
