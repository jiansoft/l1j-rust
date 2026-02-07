/// Character creation response packets.

use crate::protocol::opcodes::server;
use crate::protocol::packet::PacketBuilder;

/// S_CharCreateStatus reason codes.
pub const REASON_OK: u8 = 0x02;
pub const REASON_ALREADY_EXISTS: u8 = 0x06;
pub const REASON_INVALID_NAME: u8 = 0x09;
pub const REASON_WRONG_AMOUNT: u8 = 0x15;

/// Build S_CharCreateStatus (S_NewCharWrong) - creation result.
pub fn build_char_create_status(reason: u8) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_NEWCHARWRONG)
        .write_c(reason as i32)
        .write_d(0)
        .write_h(0)
        .build()
}

/// Build S_NewCharPacket - sends the newly created character's data.
pub fn build_new_char_pack(
    name: &str,
    char_type: i32,
    sex: i32,
    lawful: i32,
    max_hp: i32,
    max_mp: i32,
    ac: i32,
    level: i32,
    str_s: i32,
    dex_s: i32,
    con_s: i32,
    wis_s: i32,
    cha_s: i32,
    int_s: i32,
    birthday: i32,
) -> Vec<u8> {
    let checksum = (level ^ str_s ^ dex_s ^ con_s ^ wis_s ^ cha_s ^ int_s) & 0xFF;

    PacketBuilder::new(server::S_OPCODE_NEWCHARPACK)
        .write_s(Some(name))
        .write_s(Some(""))     // clan name (empty)
        .write_c(char_type)
        .write_c(sex)
        .write_h(lawful)
        .write_h(max_hp)
        .write_h(max_mp)
        .write_c(ac)
        .write_c(level)
        .write_c(str_s)
        .write_c(dex_s)
        .write_c(con_s)
        .write_c(wis_s)
        .write_c(cha_s)
        .write_c(int_s)
        .write_c(0)            // admin flag
        .write_d(birthday)
        .write_c(checksum)
        .build()
}
