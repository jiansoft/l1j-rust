use crate::protocol::opcodes::server;
use crate::protocol::packet::PacketBuilder;

/// Build S_NORMALCHAT (normal/shout chat).
pub fn build_normal_chat(
    object_id: i32,
    chat_type: i32,
    name: &str,
    message: &str,
) -> Vec<u8> {
    let text = match chat_type {
        0 => format!("{}: {}", name, message),   // Normal
        2 => format!("<{}> {}", name, message),   // Shout
        _ => format!("{}: {}", name, message),
    };

    let mut pb = PacketBuilder::new(server::S_OPCODE_NORMALCHAT)
        .write_c(chat_type);

    if chat_type == 2 {
        // Shout includes coordinates (but we write 0 for now)
        pb = pb.write_d(object_id)
            .write_s(Some(&text))
            .write_h(0) // X
            .write_h(0); // Y
    } else {
        pb = pb.write_d(object_id)
            .write_s(Some(&text));
    }

    pb.build()
}

/// Build S_GLOBALCHAT (world/clan/alliance chat).
pub fn build_global_chat(
    chat_type: i32,
    name: &str,
    message: &str,
) -> Vec<u8> {
    let text = match chat_type {
        3 => format!("[{}] {}", name, message),   // World
        4 => format!("{{{}}} {}", name, message), // Clan
        _ => format!("{}: {}", name, message),
    };

    PacketBuilder::new(server::S_OPCODE_GLOBALCHAT)
        .write_c(chat_type)
        .write_s(Some(&text))
        .build()
}

/// Build S_SERVERMSG - server system message.
pub fn build_server_message(message: &str) -> Vec<u8> {
    PacketBuilder::new(server::S_OPCODE_SERVERMSG)
        .write_c(9)              // message type
        .write_d(0)              // msg id (0 = custom string)
        .write_s(Some(message))
        .build()
}
