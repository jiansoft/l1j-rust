/// Skill-related server packets.

use crate::protocol::opcodes::server;
use crate::protocol::packet::PacketBuilder;

/// Build S_ADDSKILL - adds a skill to the player's skill list.
pub fn build_add_skill(skill_number: i32) -> Vec<u8> {
    // Skill list is sent as a bitmask: each bit = one skill learned
    // For simplicity, we send individual skill additions
    PacketBuilder::new(server::S_OPCODE_ADDSKILL)
        .write_c(skill_number)
        .write_c(0) // padding
        .build()
}

/// Build S_SKILLSOUNDGFX - visual effect at caster.
pub fn build_skill_sound(object_id: i32, gfx_id: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_SKILLSOUNDGFX)
        .write_d(object_id)
        .write_h(gfx_id)
        .build()
}

/// Build S_EFFECTLOCATION - visual effect at coordinates.
pub fn build_effect_location(x: i32, y: i32, gfx_id: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_EFFECTLOCATION)
        .write_h(x)
        .write_h(y)
        .write_h(gfx_id)
        .build()
}

/// Build S_SKILLHASTE - haste/slow effect icon.
pub fn build_skill_haste(object_id: i32, haste_type: i32, duration: i32) -> Vec<u8> {
    // haste_type: 1=haste, 2=slow (client shows different icon)
    PacketBuilder::new(server::S_OPCODE_SKILLHASTE)
        .write_d(object_id)
        .write_c(haste_type)
        .write_h(duration) // duration in seconds
        .build()
}

/// Build S_SKILLBRAVE - brave potion / buff icon.
pub fn build_skill_brave(object_id: i32, brave_type: i32, duration: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_SKILLBRAVE)
        .write_d(object_id)
        .write_c(brave_type)
        .write_h(duration)
        .build()
}

/// Build S_SKILLICONSHIELD - defense buff icon (AC bonus etc).
pub fn build_skill_icon_shield(shield_type: i32, duration: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_SKILLICONSHIELD)
        .write_c(shield_type)
        .write_h(duration)
        .build()
}

/// Build S_PARALYSIS - paralysis/freeze/sleep effect.
/// state: 1=paralyze, 2=stun, 3=sleep, 4=freeze
pub fn build_paralysis(state: i32, is_start: bool) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_PARALYSIS)
        .write_c(state)
        .write_c(if is_start { 1 } else { 0 })
        .build()
}
