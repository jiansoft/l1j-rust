/// Teleport system - full execution logic.
///
/// Ported from Java L1Teleport.java + Teleportation.java.
/// Handles portal entry, bookmark teleport, scroll teleport, etc.

use crate::protocol::opcodes::server;
use crate::protocol::packet::PacketBuilder;

/// Build the S_MAPID packet for map transition.
pub fn build_map_id(map_id: i32, is_underwater: bool) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_MAPID)
        .write_h(map_id)
        .write_c(if is_underwater { 1 } else { 0 })
        .write_d(0)
        .write_d(0)
        .write_d(0)
        .build()
}

/// Build S_BOOKMARKS packet - adds a bookmark entry on the client.
pub fn build_bookmark(name: &str, map_id: i32, bookmark_id: i32, x: i32, y: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_BOOKMARKS)
        .write_s(Some(name))
        .write_h(map_id)
        .write_d(bookmark_id)
        .write_h(x)
        .write_h(y)
        .build()
}

/// Describes all state changes and packets needed for a teleportation.
///
/// The caller (game engine / session handler) executes these steps:
///   1. Remove player from old map's visibility
///   2. Update player position in WorldGrid
///   3. Send packets to the player
///   4. Send remove/add packets to nearby players on both maps
///   5. Teleport pets/summons if allowed
///   6. Clear player's known object list
#[derive(Debug)]
pub struct TeleportAction {
    /// Packets to send to the teleporting player, in order.
    pub player_packets: Vec<Vec<u8>>,
    /// New position after teleport.
    pub new_x: i32,
    pub new_y: i32,
    pub new_map_id: i32,
    pub new_heading: i32,
    /// Whether to show teleport animation/sound.
    pub show_effect: bool,
}

/// Build a full teleport action from portal entry.
///
/// Equivalent to Java `L1Teleport.teleport()` + `Teleportation.actionTeleportation()`.
pub fn build_portal_teleport(
    object_id: i32,
    new_x: i32,
    new_y: i32,
    new_map_id: i32,
    new_heading: i32,
    gfxid: i32,
    name: &str,
    clan_name: &str,
    lawful: i32,
    is_underwater: bool,
) -> TeleportAction {
    let mut packets = Vec::new();

    // 1. S_MAPID - tell client the new map
    packets.push(build_map_id(new_map_id, is_underwater));

    // 2. S_OWNCHARPACK - reposition character
    packets.push(build_own_char_at(
        object_id, new_x, new_y, gfxid, new_heading, name, clan_name, lawful,
    ));

    TeleportAction {
        player_packets: packets,
        new_x,
        new_y,
        new_map_id,
        new_heading,
        show_effect: false,
    }
}

/// Build a teleport action with visual effect (spell teleport, scroll, etc).
pub fn build_effect_teleport(
    object_id: i32,
    new_x: i32,
    new_y: i32,
    new_map_id: i32,
    new_heading: i32,
    gfxid: i32,
    name: &str,
    clan_name: &str,
    lawful: i32,
    is_underwater: bool,
) -> TeleportAction {
    let mut packets = Vec::new();

    // 1. Teleport sound effect (gfx 169 = blue teleport animation)
    packets.push(
        PacketBuilder::new(server::S_OPCODE_SKILLSOUNDGFX)
            .write_d(object_id)
            .write_h(169)
            .build(),
    );

    // 2. S_MAPID
    packets.push(build_map_id(new_map_id, is_underwater));

    // 3. S_OWNCHARPACK
    packets.push(build_own_char_at(
        object_id, new_x, new_y, gfxid, new_heading, name, clan_name, lawful,
    ));

    TeleportAction {
        player_packets: packets,
        new_x,
        new_y,
        new_map_id,
        new_heading,
        show_effect: true,
    }
}

/// Helper: build S_OWNCHARPACK at a specific position.
fn build_own_char_at(
    object_id: i32,
    x: i32,
    y: i32,
    gfxid: i32,
    heading: i32,
    name: &str,
    clan_name: &str,
    lawful: i32,
) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_CHARPACK)
        .write_h(x)
        .write_h(y)
        .write_d(object_id)
        .write_h(gfxid)
        .write_c(0)              // weapon
        .write_c(heading)
        .write_c(0)              // light
        .write_c(0)              // speed
        .write_d(1)              // exp
        .write_h(lawful)
        .write_s(Some(name))
        .write_s(Some(""))       // title
        .write_c(4)              // STATUS_PC
        .write_d(0)              // emblem
        .write_s(Some(clan_name))
        .write_s(None)
        .write_c(0xb0_u8 as i32) // clan rank
        .write_c(0xff_u8 as i32) // party hp
        .write_c(0)
        .write_c(0)
        .write_c(0)
        .write_c(0xff_u8 as i32)
        .write_c(0xff_u8 as i32)
        .write_s(None)
        .write_c(0)
        .build()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portal_teleport_generates_packets() {
        let action = build_portal_teleport(
            100, 32800, 32800, 4, 0, 61, "TestChar", "", 0, false,
        );
        assert_eq!(action.player_packets.len(), 2); // MAPID + CHARPACK
        assert_eq!(action.new_x, 32800);
        assert_eq!(action.new_map_id, 4);
        assert!(!action.show_effect);
    }

    #[test]
    fn test_effect_teleport_has_sound() {
        let action = build_effect_teleport(
            100, 32800, 32800, 4, 0, 61, "TestChar", "", 0, false,
        );
        assert_eq!(action.player_packets.len(), 3); // SOUND + MAPID + CHARPACK
        assert!(action.show_effect);
    }

    #[test]
    fn test_bookmark_packet() {
        let pkt = build_bookmark("我的村莊", 4, 12345, 32800, 32800);
        assert!(!pkt.is_empty());
        assert_eq!(pkt[0], server::S_OPCODE_BOOKMARKS);
    }
}
