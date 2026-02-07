/// NPC dialogue and shop packets.
///
/// L1J uses HTML-based dialogue windows. The server sends
/// an HTML string which the client renders in a popup.

use crate::protocol::opcodes::server;
use crate::protocol::packet::PacketBuilder;

/// Build S_SHOWHTML - opens an HTML dialogue window.
///
/// The `html` string can contain L1J-specific HTML tags for
/// buttons, links, images, etc.
pub fn build_show_html(object_id: i32, html: &str) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_SHOWHTML)
        .write_d(object_id)
        .write_s(Some(html))
        .build()
}

/// Build S_SHOWSHOPBUYLIST - shows a shop's buy (sell to player) list.
///
/// Each entry: item_id, price, pack_count
pub fn build_shop_buy_list(
    npc_object_id: i32,
    items: &[(i32, i32, i32)], // (item_id, price, pack_count)
) -> Vec<u8> {
    let mut pb = PacketBuilder::new(server::S_OPCODE_SHOWSHOPBUYLIST)
        .write_d(npc_object_id)
        .write_h(items.len() as i32);

    for &(item_id, price, pack_count) in items {
        pb = pb.write_d(item_id)
            .write_d(price)
            .write_d(pack_count);
    }

    pb.build()
}

/// Build S_SHOWSHOPSELLLIST - shows what the shop will buy from the player.
pub fn build_shop_sell_list(npc_object_id: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_SHOWSHOPSELLLIST)
        .write_d(npc_object_id)
        .build()
}

/// Build S_SELECTLIST - shows a list of items for repair/enchant.
pub fn build_select_list(
    npc_object_id: i32,
    items: &[(i32, String)], // (item_object_id, item_name)
) -> Vec<u8> {
    let mut pb = PacketBuilder::new(server::S_OPCODE_SELECTLIST)
        .write_d(npc_object_id)
        .write_h(items.len() as i32);

    for (obj_id, name) in items {
        pb = pb.write_d(*obj_id)
            .write_s(Some(name));
    }

    pb.build()
}

/// Build S_YES_NO - shows a confirmation dialog.
///
/// The client will respond with C_ATTR (opcode 121) with the result.
pub fn build_yes_no(message_id: i32, yes_no_id: i32) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_YES_NO)
        .write_h(yes_no_id)
        .write_d(message_id)
        .build()
}

/// Build a simple NPC chat HTML page.
///
/// This is a convenience function for common dialogue patterns.
pub fn build_simple_dialog(npc_name: &str, message: &str, buttons: &[(&str, &str)]) -> String {
    let mut html = format!(
        "<html><body><center><b>{}</b></center><br>{}<br><br>",
        npc_name, message
    );
    for (label, action) in buttons {
        html.push_str(&format!(
            "<a action=\"{}\">{}</a><br>",
            action, label
        ));
    }
    html.push_str("</body></html>");
    html
}
