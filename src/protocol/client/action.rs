use crate::protocol::packet::PacketReader;

/// Parsed C_ATTACK packet (melee attack).
pub struct Attack {
    pub target_id: i32,
    pub x: i32,
    pub y: i32,
}

pub fn parse_attack(data: &[u8]) -> Attack {
    let mut r = PacketReader::after_opcode(data);
    let target_id = r.read_d();
    let x = r.read_h() as i32;
    let y = r.read_h() as i32;
    Attack { target_id, x, y }
}

/// Parsed C_ARROWATTACK packet (ranged attack).
pub struct ArrowAttack {
    pub target_id: i32,
    pub x: i32,
    pub y: i32,
}

pub fn parse_arrow_attack(data: &[u8]) -> ArrowAttack {
    let mut r = PacketReader::after_opcode(data);
    let target_id = r.read_d();
    let x = r.read_h() as i32;
    let y = r.read_h() as i32;
    ArrowAttack { target_id, x, y }
}

/// Parsed C_PICKUPITEM packet.
pub struct PickUpItem {
    pub x: i32,
    pub y: i32,
    pub object_id: i32,
    pub count: i32,
}

pub fn parse_pickup_item(data: &[u8]) -> PickUpItem {
    let mut r = PacketReader::after_opcode(data);
    let x = r.read_h() as i32;
    let y = r.read_h() as i32;
    let object_id = r.read_d();
    let count = r.read_d();
    PickUpItem { x, y, object_id, count }
}

/// Parsed C_USEITEM packet.
pub struct UseItem {
    pub item_obj_id: i32,
}

pub fn parse_use_item(data: &[u8]) -> UseItem {
    let mut r = PacketReader::after_opcode(data);
    let item_obj_id = r.read_d();
    UseItem { item_obj_id }
}
