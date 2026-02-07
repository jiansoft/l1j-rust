use crate::db::character::CharacterListData;
use crate::protocol::opcodes::server;
use crate::protocol::packet::PacketBuilder;

/// Build S_CHARAMOUNT packet.
///
/// Tells the client how many characters the account has and the max slots.
pub fn build_char_amount(count: i32, max_slots: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_CHARAMOUNT)
        .write_c(count)
        .write_c(max_slots)
        .build()
}

/// Build S_CHARSYNACK (SYN) packet - marks start of character list.
pub fn build_char_syn() -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_CHARSYNACK)
        .write_c(0x0a)  // SYN type
        .write_c(0x02)
        .write_c(0x00)
        .write_c(0x00)
        .write_c(0x00)
        .write_c(0x08)
        .write_c(0x00)
        .build()
}

/// Build S_CHARSYNACK (ACK) packet - marks end of character list.
pub fn build_char_ack() -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_CHARSYNACK)
        .write_c(0x40)  // ACK type
        .write_d(0x00000000)
        .write_h(0x0000)
        .build()
}

/// Build S_CHARLIST packet for a single character.
///
/// One of these is sent per character in the account's character list.
pub fn build_char_pack(ch: &CharacterListData) -> Vec<u8> {
    let checksum = (ch.level ^ ch.str_stat ^ ch.dex_stat ^ ch.con_stat
        ^ ch.wis_stat ^ ch.cha_stat ^ ch.int_stat)
        & 0xFF;

    PacketBuilder::new(server::S_OPCODE_CHARLIST)
        .write_s(Some(&ch.char_name))
        .write_s(if ch.clanname.is_empty() {
            Some("")
        } else {
            Some(&ch.clanname)
        })
        .write_c(ch.char_type)
        .write_c(ch.sex)
        .write_h(ch.lawful)
        .write_h(ch.cur_hp)
        .write_h(ch.cur_mp)
        .write_c(ch.ac)
        .write_c(ch.level)
        .write_c(ch.str_stat)
        .write_c(ch.dex_stat)
        .write_c(ch.con_stat)
        .write_c(ch.wis_stat)
        .write_c(ch.cha_stat)
        .write_c(ch.int_stat)
        .write_c(0) // admin flag
        .write_d(ch.birthday)
        .write_c(checksum)
        .build()
}

/// Build S_OWNCHARSTATUS packet.
///
/// Sends full character status after entering the game world.
pub fn build_own_char_status(ch: &crate::db::character::CharacterFullData) -> Vec<u8> {
    let game_time = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / 300
        * 300) as i32;

    PacketBuilder::new(server::S_OPCODE_OWNCHARSTATUS)
        .write_d(ch.objid)
        .write_c(ch.level.min(127).max(1))
        .write_d(ch.exp)
        .write_c(ch.str_stat)
        .write_c(ch.int_stat)
        .write_c(ch.wis_stat)
        .write_c(ch.dex_stat)
        .write_c(ch.con_stat)
        .write_c(ch.cha_stat)
        .write_h(ch.cur_hp)
        .write_h(ch.max_hp)
        .write_h(ch.cur_mp)
        .write_h(ch.max_mp)
        .write_c(ch.ac)
        .write_d(game_time)
        .write_c(ch.food)        // food
        .write_c(0)              // weight (placeholder)
        .write_h(ch.lawful)
        .write_h(0)              // fire
        .write_h(0)              // water
        .write_h(0)              // wind
        .write_h(0)              // earth
        .write_d(ch.mons_kill)
        .build()
}

/// Build S_MAPID packet.
///
/// Tells the client which map the character is on.
pub fn build_map_id(map_id: i32, is_underwater: bool) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_MAPID)
        .write_h(map_id)
        .write_c(if is_underwater { 1 } else { 0 })
        .write_d(0)
        .write_d(0)
        .write_d(0)
        .build()
}

/// Build S_OWNCHARPACK packet.
///
/// Character appearance packet sent when entering the game.
pub fn build_own_char_pack(ch: &crate::db::character::CharacterFullData) -> Vec<u8> {
    let status: i32 = 4; // STATUS_PC = 4

    PacketBuilder::new(server::S_OPCODE_CHARPACK)
        .write_h(ch.loc_x)
        .write_h(ch.loc_y)
        .write_d(ch.objid)
        .write_h(ch.gfxid)       // character GFX
        .write_c(0)              // current weapon
        .write_c(ch.heading)
        .write_c(0)              // light size
        .write_c(0)              // move speed
        .write_d(1)              // exp (hardcoded to 1)
        .write_h(ch.lawful)
        .write_s(Some(&ch.char_name))
        .write_s(Some(""))       // title
        .write_c(status)
        .write_d(0)              // clan emblem ID (0 = no clan)
        .write_s(Some(&ch.clanname))
        .write_s(None)           // padding
        .write_c(0xb0_u8 as i32) // clan rank (no clan)
        .write_c(0xff_u8 as i32) // party HP% (not in party)
        .write_c(0)              // third speed
        .write_c(0)              // PC level (0 for PC)
        .write_c(0)              // unknown
        .write_c(0xff_u8 as i32) // padding
        .write_c(0xff_u8 as i32) // padding
        .write_s(None)           // padding
        .write_c(0)              // padding
        .build()
}
