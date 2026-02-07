/// Clan server packets - complete set.
///
/// Ported from Java S_ClanName, S_War, S_CastleMaster, S_Emblem,
/// S_Pledge, S_PacketBox (clan related).

use crate::protocol::opcodes::server;
use crate::protocol::packet::PacketBuilder;

/// Build S_CLANNAME - sets the clan name display above a character.
pub fn build_clan_name(object_id: i32, clan_name: &str, show: bool) -> Vec<u8> {
    if show {
        PacketBuilder::new(server::S_OPCODE_CLANNAME)
            .write_d(object_id)
            .write_s(Some(clan_name))
            .build()
    } else {
        PacketBuilder::new(server::S_OPCODE_CLANNAME)
            .write_d(object_id)
            .write_s(Some(""))
            .build()
    }
}

/// Build S_WAR - war declaration/surrender/cease notification.
/// war_type: 1=clan war, 2=siege war
pub fn build_war(war_type: i32, clan1: &str, clan2: &str) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_WAR)
        .write_c(war_type)
        .write_s(Some(clan1))
        .write_s(Some(clan2))
        .build()
}

/// Build S_CASTLEMASTER - shows the crown icon for castle lords.
pub fn build_castle_master(object_id: i32, castle_id: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_CASTLEMASTER)
        .write_d(object_id)
        .write_c(castle_id)
        .build()
}

/// Build S_EMBLEM - sends clan emblem image data.
pub fn build_emblem(clan_id: i32, emblem_data: &[u8]) -> Vec<u8> {
    let pb = PacketBuilder::new(server::S_OPCODE_EMBLEM)
        .write_d(clan_id)
        .write_d(emblem_data.len() as i32);

    let mut buf = pb.build();
    buf.extend_from_slice(emblem_data);
    buf
}

/// Build S_CHARTITLE - updates a character's title display.
pub fn build_char_title(object_id: i32, title: &str) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_CHARTITLE)
        .write_d(object_id)
        .write_s(Some(title))
        .build()
}

/// Build S_SERVERMSG for clan-related system messages.
///
/// Common message IDs (from Java):
///   84  = "創立 %0 血盟。"
///   85  = "只有王族可以創立血盟。"
///   86  = "你已經加入血盟了。"
///   89  = "你已經在血盟裡了。"
///   90  = "%0 沒有血盟。"
///   92  = "%0 不是王族。"
///   99  = "已經有同名的血盟了。"
///   109 = "沒有叫 %0 的人。"
///   178 = "%0 離開了 %1 血盟。"
///   189 = "金幣不足。"
///   238 = "你被 %0 血盟驅逐了。"
///   240 = "%0 被你從血盟驅逐了。"
///   269 = "%0 解散了 %1 血盟。"
///   518 = "只有血盟君主可以使用。"
pub fn build_system_message(msg_id: i32, args: &[&str]) -> Vec<u8> {
    let mut pb = PacketBuilder::new(server::S_OPCODE_SERVERMSG)
        .write_c(9)
        .write_d(msg_id);

    for arg in args {
        pb = pb.write_s(Some(arg));
    }

    pb.build()
}
