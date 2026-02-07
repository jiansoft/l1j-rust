/// Skill visual effect packets.
///
/// S_UseAttackSkill (opcode 30) - single target attack animation
/// S_RangeSkill (opcode 42) - AoE skill animation
/// S_SkillSound (opcode 55) - buff/effect animation on entity

use crate::protocol::opcodes::server;
use crate::protocol::packet::PacketBuilder;

/// Build S_UseAttackSkill - single target attack spell visual.
pub fn build_attack_skill(
    caster_id: i32,
    target_id: i32,
    damage: i32,
    heading: i32,
    gfx_id: i32,
    caster_x: i32,
    caster_y: i32,
    target_x: i32,
    target_y: i32,
    seq_number: i32,
    with_cast_motion: bool,
) -> Vec<u8> {
    let action_id = 19; // ACTION_SkillAttack
    PacketBuilder::new(server::S_OPCODE_ATTACKPACKET)
        .write_c(action_id)
        .write_d(if with_cast_motion { caster_id } else { 0 })
        .write_d(target_id)
        .write_h(damage)
        .write_c(heading)
        .write_d(seq_number)
        .write_h(gfx_id)
        .write_c(6)            // useType: 6 = ranged magic
        .write_h(caster_x)
        .write_h(caster_y)
        .write_h(target_x)
        .write_h(target_y)
        .write_c(0)
        .write_c(0)
        .write_c(0)            // effectFlags
        .build()
}

/// Build S_RangeSkill - AoE skill visual.
///
/// `range_type`: 0 = non-directional (meteor), 8 = directional (lightning)
pub fn build_range_skill(
    caster_id: i32,
    caster_x: i32,
    caster_y: i32,
    heading: i32,
    gfx_id: i32,
    range_type: i32,
    target_ids: &[i32],
    seq_number: i32,
) -> Vec<u8> {
    let action_id = 19;
    let mut pb = PacketBuilder::new(server::S_OPCODE_RANGESKILLS)
        .write_c(action_id)
        .write_d(caster_id)
        .write_h(caster_x)
        .write_h(caster_y)
        .write_c(heading)
        .write_d(seq_number)
        .write_h(gfx_id)
        .write_c(range_type)
        .write_h(0)             // padding
        .write_h(target_ids.len() as i32);

    for &tid in target_ids {
        pb = pb.write_d(tid)
            .write_h(0x20);     // damage flag
    }

    pb.build()
}

/// Build S_SkillSound - visual/sound effect on an entity.
pub fn build_skill_sound(object_id: i32, gfx_id: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_SKILLSOUNDGFX)
        .write_d(object_id)
        .write_h(gfx_id)
        .build()
}
