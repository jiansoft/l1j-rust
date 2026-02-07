/// S_INITPACKET - Handshake packet builder.
///
/// This is the very first packet sent by the server when a client connects.
/// It is NOT encrypted (cipher hasn't been initialized yet).
///
/// Note: The actual handshake sending is done in session.rs.
/// This module provides helper functions for building init-related packets.

use crate::protocol::opcodes::server;

/// Build the S_SERVERVERSION packet.
///
/// Sent in response to C_CLIENTVERSION.
/// TODO Phase 2: Fill in actual server version data.
pub fn build_server_version() -> Vec<u8> {
    let mut buf = Vec::with_capacity(32);
    buf.push(server::S_OPCODE_SERVERVERSION);
    // TODO: Write actual version fields
    //   writeC(0x00)  // auto_create_accounts
    //   writeC(0x00)  // server type
    //   writeC(0x00)  // country code
    //   writeD(...)   // server version
    //   writeD(...)   // cache version
    //   writeD(...)   // auth version
    //   writeD(...)   // npc version
    buf
}

/// Build the S_LOGINRESULT packet.
///
/// Sent after account authentication attempt.
pub fn build_login_result(reason: u8) -> Vec<u8> {
    let mut buf = Vec::with_capacity(4);
    buf.push(server::S_OPCODE_LOGINRESULT);
    buf.push(reason);
    buf.push(0);
    buf.push(0);
    buf
}
