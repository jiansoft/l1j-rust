/// All packets required when a character enters the game world.
///
/// These must be sent in order after S_LoginGame, otherwise
/// the client will freeze on the loading screen or disconnect.

use crate::db::character::CharacterFullData;
use crate::protocol::opcodes::server;
use crate::protocol::packet::PacketBuilder;

/// Build S_SPMR - spell power and magic resistance from equipment.
pub fn build_spmr(equipment_sp: i32, equipment_mr: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_SPMR)
        .write_c(equipment_sp)
        .write_h(equipment_mr)
        .build()
}

/// Build S_OwnCharStatus2 - base stat display (type=1).
pub fn build_own_char_status2(ch: &CharacterFullData) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_OWNCHARSTATUS2)
        .write_c(ch.str_stat)
        .write_c(ch.int_stat)
        .write_c(ch.wis_stat)
        .write_c(ch.dex_stat)
        .write_c(ch.con_stat)
        .write_c(ch.cha_stat)
        .write_c(0) // weight (placeholder)
        .build()
}

/// Build S_InitialAbilityGrowth - initial stat allocation display.
pub fn build_initial_ability(ch: &CharacterFullData) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_CHARSYNACK) // opcode 64
        .write_c(0x04)
        .write_c((ch.int_stat * 16) + ch.str_stat)
        .write_c((ch.dex_stat * 16) + ch.wis_stat)
        .write_c((ch.cha_stat * 16) + ch.con_stat)
        .write_c(0x00)
        .build()
}

/// Build S_Weather - current weather.
pub fn build_weather(weather: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_WEATHER)
        .write_c(weather) // 0=clear, 1=snow, 2=rain, 4=??
        .build()
}

/// Build S_Light - light radius around character.
pub fn build_light(object_id: i32, light_size: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_LIGHT)
        .write_d(object_id)
        .write_c(light_size)
        .build()
}

/// Build S_GameTime - current in-game time.
pub fn build_game_time() -> Vec<u8> {
    let time = (std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs()
        / 300
        * 300) as i32;

    PacketBuilder::new(server::S_OPCODE_GAMETIME)
        .write_d(time)
        .build()
}

/// Build S_AddSkill - player's learned skill bitmask.
///
/// For a new character with no skills, send all zeros.
/// 28 bytes of skill level bitmasks.
pub fn build_add_skill_empty() -> Vec<u8> {
    let mut pb = PacketBuilder::new(server::S_OPCODE_ADDSKILL)
        .write_c(32); // size marker

    // 28 skill level bytes (all 0 = no skills learned)
    for _ in 0..28 {
        pb = pb.write_c(0);
    }
    pb = pb.write_d(0).write_d(0);

    pb.build()
}

/// Build S_OwnCharAttrDef - character attribute defense values.
pub fn build_own_char_attr_def(_ch: &CharacterFullData) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_OWNCHARATTRDEF)
        .write_h(0) // fire def
        .write_h(0) // water def
        .write_h(0) // wind def
        .write_h(0) // earth def
        .build()
}

/// Build S_Karma - character's karma value.
pub fn build_karma(karma: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_PACKETBOX)
        .write_c(0x57) // karma subcode
        .write_d(karma)
        .build()
}

/// Build S_HPUpdate - initial HP display.
pub fn build_hp_update(cur_hp: i32, max_hp: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_HPUPDATE)
        .write_h(cur_hp.clamp(1, 32767))
        .write_h(max_hp.clamp(1, 32767))
        .build()
}

/// Build S_MPUpdate - initial MP display.
pub fn build_mp_update(cur_mp: i32, max_mp: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_MPUPDATE)
        .write_h(cur_mp)
        .write_h(max_mp)
        .build()
}

/// Build S_Ability - movement speed ability packet.
pub fn build_ability(_char_type: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_ABILITY)
        .write_c(0) // teleport ring count
        .write_h(0) // unknown
        .write_c(0) // unknown
        .write_d(0) // unknown
        .build()
}

/// Send ALL required init packets for entering the game world.
///
/// Returns a Vec of packet byte arrays to send in order.
pub fn build_all_game_init_packets(
    ch: &CharacterFullData,
    weather: i32,
) -> Vec<Vec<u8>> {
    let mut packets = Vec::with_capacity(20);

    // 1. S_LoginGame
    packets.push(
        crate::protocol::server::login::build_login_to_game(ch.clanid, 0),
    );

    // 2. S_OwnCharStatus
    packets.push(
        crate::protocol::server::char_list::build_own_char_status(ch),
    );

    // 3. S_MapID
    packets.push(
        crate::protocol::server::char_list::build_map_id(ch.map_id, false),
    );

    // 4. S_OwnCharPack
    packets.push(
        crate::protocol::server::char_list::build_own_char_pack(ch),
    );

    // 5. S_SPMR
    packets.push(build_spmr(0, 0));

    // 6. S_OwnCharStatus2
    packets.push(build_own_char_status2(ch));

    // 7. S_InitialAbilityGrowth
    packets.push(build_initial_ability(ch));

    // 8. S_Weather
    packets.push(build_weather(weather));

    // 9. S_AddSkill (empty for now)
    packets.push(build_add_skill_empty());

    // 10. S_Light
    packets.push(build_light(ch.objid, 0));

    // 11. S_GameTime
    packets.push(build_game_time());

    // 12. S_Karma
    packets.push(build_karma(0));

    // 13. S_HPUpdate
    packets.push(build_hp_update(ch.cur_hp, ch.max_hp));

    // 14. S_MPUpdate
    packets.push(build_mp_update(ch.cur_mp, ch.max_mp));

    // 15. S_OwnCharAttrDef
    packets.push(build_own_char_attr_def(ch));

    // 16. S_Ability
    packets.push(build_ability(ch.char_type));

    // 17. S_InvList (empty inventory for now - real items loaded separately)
    packets.push(
        PacketBuilder::new(server::S_OPCODE_INVLIST)
            .write_c(0)  // 0 items
            .build(),
    );

    packets
}
