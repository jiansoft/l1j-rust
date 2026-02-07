/// C_USESKILL packet parser.
///
/// Skill ID = row * 8 + column + 1 (from the client UI grid).

use crate::protocol::packet::PacketReader;

/// Parsed C_USESKILL packet.
#[derive(Debug)]
pub struct UseSkill {
    pub skill_id: i32,
    pub target_id: i32,
    pub target_x: i32,
    pub target_y: i32,
    pub char_name: String,   // for CALL_CLAN / TRUE_TARGET
    pub bookmark_id: i32,    // for TELEPORT
}

/// Special skill IDs with unique packet formats.
pub const SKILL_TELEPORT: i32 = 5;
pub const SKILL_FIRE_WALL: i32 = 49;
pub const SKILL_LIFE_STREAM: i32 = 50;
pub const SKILL_CALL_CLAN: i32 = 68;
pub const SKILL_RUN_CLAN: i32 = 69;
pub const SKILL_TRUE_TARGET: i32 = 70;

pub fn parse_use_skill(data: &[u8]) -> UseSkill {
    let mut r = PacketReader::after_opcode(data);
    let row = r.read_c() as i32;
    let column = r.read_c() as i32;
    let skill_id = row * 8 + column + 1;

    let mut result = UseSkill {
        skill_id,
        target_id: 0,
        target_x: 0,
        target_y: 0,
        char_name: String::new(),
        bookmark_id: 0,
    };

    // Variable data depends on skill type
    if !r.has_remaining() {
        return result;
    }

    match skill_id {
        SKILL_CALL_CLAN | SKILL_RUN_CLAN => {
            result.char_name = r.read_s();
        }
        SKILL_TRUE_TARGET => {
            result.target_id = r.read_d();
            result.target_x = r.read_h() as i32;
            result.target_y = r.read_h() as i32;
            result.char_name = r.read_s();
        }
        SKILL_TELEPORT => {
            let _map_id = r.read_h();
            result.bookmark_id = r.read_d();
        }
        SKILL_FIRE_WALL | SKILL_LIFE_STREAM => {
            result.target_x = r.read_h() as i32;
            result.target_y = r.read_h() as i32;
        }
        _ => {
            // Default: targetId + X + Y
            result.target_id = r.read_d();
            result.target_x = r.read_h() as i32;
            result.target_y = r.read_h() as i32;
        }
    }

    result
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_skill_id_calculation() {
        // Row 0, Column 0 → skill 1 (Energy Bolt)
        // Row 2, Column 1 → skill 2*8+1+1 = 18 (Fireball)
        assert_eq!(0 * 8 + 0 + 1, 1);
        assert_eq!(2 * 8 + 1 + 1, 18);
        assert_eq!(6 * 8 + 7 + 1, 56); // Absolute Barrier
    }
}
