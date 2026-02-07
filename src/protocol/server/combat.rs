use crate::protocol::opcodes::server;
use crate::protocol::packet::PacketBuilder;

/// Action IDs for combat animations.
pub const ACTION_IDLE: i32 = 0;
pub const ACTION_ATTACK: i32 = 1;
pub const ACTION_DAMAGE: i32 = 2;
pub const ACTION_HIDE: i32 = 4;
pub const ACTION_DIE: i32 = 8;
pub const ACTION_PICKUP: i32 = 15;

/// Attack effect flags.
pub const EFFECT_NONE: i32 = 0;
pub const EFFECT_CLAW: i32 = 2;
pub const EFFECT_DOUBLE_HIT: i32 = 4;
pub const EFFECT_MIRROR: i32 = 8;

/// Build S_ATTACKPACKET - shows attack animation + damage number.
pub fn build_attack_packet(
    attacker_id: i32,
    target_id: i32,
    action_id: i32,
    damage: i32,
    heading: i32,
    effect: i32,
) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_ATTACKPACKET)
        .write_c(action_id)
        .write_d(attacker_id)
        .write_d(target_id)
        .write_h(damage)
        .write_c(heading)
        .write_d(0)            // padding
        .write_c(effect)
        .build()
}

/// Build S_DOACTIONGFX - plays an action animation on an entity.
pub fn build_do_action_gfx(object_id: i32, action_id: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_DOACTIONGFX)
        .write_d(object_id)
        .write_c(action_id)
        .build()
}

/// Build S_HPUPDATE - updates the player's own HP display.
pub fn build_hp_update(cur_hp: i32, max_hp: i32) -> Vec<u8> {
    let hp = cur_hp.clamp(1, 32767);
    let max = max_hp.clamp(1, 32767);
    PacketBuilder::new(server::S_OPCODE_HPUPDATE)
        .write_h(hp)
        .write_h(max)
        .build()
}

/// Build S_MPUPDATE - updates the player's own MP display.
pub fn build_mp_update(cur_mp: i32, max_mp: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_MPUPDATE)
        .write_h(cur_mp)
        .write_h(max_mp)
        .build()
}

/// Build S_HPMETER - shows HP bar on another entity (NPC/player).
pub fn build_hp_meter(object_id: i32, cur_hp: i32, max_hp: i32) -> Vec<u8> {
    let ratio = if max_hp > 0 {
        (100 * cur_hp / max_hp).clamp(0, 100)
    } else {
        0
    };
    PacketBuilder::new(server::S_OPCODE_HPMETER)
        .write_d(object_id)
        .write_h(ratio)
        .build()
}
