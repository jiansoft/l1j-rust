use crate::protocol::packet::PacketReader;

/// Parsed C_CLIENTVERSION packet.
pub struct ClientVersion {
    pub client_language: i32,
    pub client_version: i32,
}

/// Parse the C_CLIENTVERSION packet.
pub fn parse_client_version(data: &[u8]) -> ClientVersion {
    let mut r = PacketReader::after_opcode(data);
    r.skip(2); // skip 2 bytes
    r.skip(1); // skip 1 byte
    let client_language = r.read_d();
    r.skip(2); // version number 1
    r.skip(2); // version number 2
    let client_version = r.read_d();
    ClientVersion {
        client_language,
        client_version,
    }
}

/// Login action codes from C_AuthLogin (opcode 210).
pub const LOGIN_ACTION_LOGIN: u8 = 0x06;
pub const LOGIN_ACTION_RETURN_CHARSELECT: u8 = 0x0b;
pub const LOGIN_ACTION_LOGOUT: u8 = 0x1c;

/// Parsed login credentials (works for both opcode 119 and 210).
pub struct AuthLogin {
    pub action: u8,
    pub account: String,
    pub password: String,
}

/// Parse C_BEANFUNLOGIN (opcode 210) - has action byte prefix.
pub fn parse_auth_login(data: &[u8]) -> AuthLogin {
    let mut r = PacketReader::after_opcode(data);
    let action = r.read_c();

    let (account, password) = if action == LOGIN_ACTION_LOGIN {
        let acc = r.read_s().to_lowercase();
        let pwd = r.read_s();
        (acc, pwd)
    } else {
        (String::new(), String::new())
    };

    AuthLogin {
        action,
        account,
        password,
    }
}

/// Parse C_LOGINPACKET (opcode 119) - direct account+password, no action byte.
pub fn parse_login_packet(data: &[u8]) -> AuthLogin {
    let mut r = PacketReader::after_opcode(data);
    let account = r.read_s().to_lowercase();
    let password = r.read_s();

    AuthLogin {
        action: LOGIN_ACTION_LOGIN,
        account,
        password,
    }
}
