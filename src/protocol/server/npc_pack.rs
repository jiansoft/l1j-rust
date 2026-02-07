/// S_NPCPack - NPC appearance packet sent to clients.
///
/// Ported from Java S_NPCPack.java. Sent when a player first sees
/// an NPC or when the NPC enters the player's screen.

use crate::ecs::components::npc::NpcTemplate;
use crate::ecs::components::position::Position;
use crate::protocol::opcodes::server;
use crate::protocol::packet::PacketBuilder;

/// Build S_NPCPack for a single NPC.
///
/// Parameters correspond to the NPC instance's current state.
pub fn build_npc_pack(
    object_id: u32,
    pos: &Position,
    template: &NpcTemplate,
    cur_hp: i32,
    max_hp: i32,
    status_flags: i32,
) -> Vec<u8> {
    let gfx_id = template.gfxid;
    let hp_percent = if max_hp > 0 {
        ((cur_hp as f32 / max_hp as f32) * 255.0) as i32
    } else {
        0xFF
    };

    PacketBuilder::new(server::S_OPCODE_CHARPACK)
        .write_h(pos.x)                    // X coordinate
        .write_h(pos.y)                    // Y coordinate
        .write_d(object_id as i32)         // Object ID
        .write_h(gfx_id)                  // GFX ID
        .write_c(0)                        // status (action)
        .write_c(pos.heading)              // heading
        .write_c(template.light_size)      // light size
        .write_c(0)                        // move speed
        .write_d(template.exp)             // exp
        .write_h(template.lawful)          // lawful
        .write_s(Some(&template.nameid))   // name ID
        .write_s(Some(""))                 // title
        .write_c(status_flags)             // status flags
        .write_d(0)                        // unknown (0 = no C_27)
        .write_s(None)                     // padding
        .write_s(None)                     // master name
        .write_c(0)                        // fly status
        .write_c(hp_percent)               // HP bar (0xFF = full)
        .write_c(0)                        // unknown
        .write_c(template.level)           // level
        .write_c(0xFF)                     // unknown
        .write_c(0xFF)                     // unknown
        .write_c(0)                        // unknown
        .build()
}

/// Build S_REMOVE_OBJECT - removes an object from the client's screen.
pub fn build_remove_object(object_id: u32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_REMOVE_OBJECT)
        .write_d(object_id as i32)
        .build()
}
