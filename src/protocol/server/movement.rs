use crate::ecs::components::position::heading_delta;
use crate::protocol::opcodes::server;
use crate::protocol::packet::PacketBuilder;

/// Build S_MOVECHARPACKET - broadcast when an entity moves.
///
/// The packet sends the PREVIOUS position + heading,
/// so the client can interpolate the movement animation.
pub fn build_move_char(object_id: i32, cur_x: i32, cur_y: i32, heading: i32) -> Vec<u8> {
    let (dx, dy) = heading_delta(heading);
    let prev_x = cur_x - dx;
    let prev_y = cur_y - dy;

    PacketBuilder::new(server::S_OPCODE_MOVEOBJECT)
        .write_d(object_id)
        .write_h(prev_x)
        .write_h(prev_y)
        .write_c(heading)
        .write_c(129)     // constant (from Java)
        .write_d(0)       // padding
        .build()
}

/// Build S_CHANGEHEADING - broadcast when entity turns without moving.
pub fn build_change_heading(object_id: i32, heading: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_CHANGEHEADING)
        .write_d(object_id)
        .write_c(heading)
        .build()
}
