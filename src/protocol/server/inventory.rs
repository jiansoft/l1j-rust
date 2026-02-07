/// Inventory-related server packets: S_AddItem, S_DeleteInventoryItem, S_InvList, S_ItemStatus.

use crate::ecs::components::item::{ItemInstance, ItemTemplate};
use crate::protocol::opcodes::server;
use crate::protocol::packet::PacketBuilder;

/// Write a single item entry (shared between S_AddItem and S_InvList).
fn write_item_entry(pb: PacketBuilder, item: &ItemInstance, template: &ItemTemplate) -> PacketBuilder {
    let view_name = item.get_view_name(template);
    let delete_flag: i32 = if item.bless >= 128 { 3 } else if template.tradable { 7 } else { 2 };

    pb.write_d(item.object_id as i32)           // object ID
        .write_h(template.ground_gfx_id)        // desc/gfx
        .write_c(template.use_type)             // use type
        .write_c(item.charge_count)             // charge count
        .write_h(template.inv_gfx_id)           // inventory GFX
        .write_c(item.bless)                    // bless
        .write_d(item.count)                    // count
        .write_c(0)                             // item status X
        .write_s(Some(&view_name))              // display name
        .write_c(0)                             // status bytes length (simplified)
        .write_c(0x17)                          // fixed value
        .write_c(0)                             // padding
        .write_h(0)                             // padding
        .write_h(0)                             // padding
        .write_c(item.enchant_level)            // enchant level
        .write_d(item.object_id as i32)         // world serial
        .write_d(0)                             // padding
        .write_d(0)                             // padding
        .write_d(delete_flag)                   // delete flag
        .write_c(0)                             // padding
}

/// Build S_ADDITEM - adds a single item to the client's inventory display.
pub fn build_add_item(item: &ItemInstance, template: &ItemTemplate) -> Vec<u8> {
    let pb = PacketBuilder::new(server::S_OPCODE_ADDITEM);
    write_item_entry(pb, item, template).build()
}

/// Build S_DELETEINVENTORYITEM - removes an item from client inventory.
pub fn build_delete_inventory_item(object_id: u32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_DELETEINVENTORYITEM)
        .write_d(object_id as i32)
        .build()
}

/// Build S_INVLIST - sends the full inventory on login.
pub fn build_inv_list(items: &[(ItemInstance, ItemTemplate)]) -> Vec<u8> {
    let mut pb = PacketBuilder::new(server::S_OPCODE_INVLIST)
        .write_c(items.len() as i32);

    for (item, template) in items {
        pb = write_item_entry(pb, item, template);
    }

    pb.build()
}

/// Build S_ITEMSTATUS - updates item display (count, enchant, name change).
pub fn build_item_status(item: &ItemInstance, template: &ItemTemplate) -> Vec<u8> {
    let view_name = item.get_view_name(template);

    PacketBuilder::new(server::S_OPCODE_ITEMSTATUS)
        .write_d(item.object_id as i32)
        .write_s(Some(&view_name))
        .write_d(item.count)
        .write_c(0)  // status bytes length (simplified)
        .build()
}
