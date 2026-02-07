/// Clan-related client packet parsers.
///
/// Ported from Java C_CreateClan, C_JoinClan, C_LeaveClan, C_BanClan, C_War, C_Pledge.

use crate::protocol::packet::PacketReader;

/// Parsed C_CREATECLAN packet.
pub struct CreateClan {
    pub clan_name: String,
}

pub fn parse_create_clan(data: &[u8]) -> CreateClan {
    let mut r = PacketReader::after_opcode(data);
    let clan_name = r.read_s();
    CreateClan { clan_name }
}

/// Parsed C_LEAVECLAN packet.
pub struct LeaveClan {
    pub clan_name: String,
}

pub fn parse_leave_clan(data: &[u8]) -> LeaveClan {
    let mut r = PacketReader::after_opcode(data);
    let clan_name = r.read_s();
    LeaveClan { clan_name }
}

/// Parsed C_BANCLAN packet (kick member).
pub struct BanClan {
    pub member_name: String,
}

pub fn parse_ban_clan(data: &[u8]) -> BanClan {
    let mut r = PacketReader::after_opcode(data);
    let member_name = r.read_s();
    BanClan { member_name }
}

/// Parsed C_WAR packet.
pub struct War {
    pub war_type: u8,        // 0=declare, 2=surrender, 3=cease
    pub enemy_clan_name: String,
}

pub fn parse_war(data: &[u8]) -> War {
    let mut r = PacketReader::after_opcode(data);
    let war_type = r.read_c();
    let enemy_clan_name = r.read_s();
    War {
        war_type,
        enemy_clan_name,
    }
}

/// Parsed C_RANK packet (set member rank).
pub struct Rank {
    pub member_name: String,
    pub rank: u8,
}

pub fn parse_rank(data: &[u8]) -> Rank {
    let mut r = PacketReader::after_opcode(data);
    let member_name = r.read_s();
    let rank = r.read_c();
    Rank { member_name, rank }
}
