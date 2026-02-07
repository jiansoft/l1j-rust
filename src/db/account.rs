use anyhow::Result;
use base64::Engine;
use sha1::{Digest, Sha1};
use sqlx::MySqlPool;

/// Account data loaded from the `accounts` MySQL table.
#[derive(Debug, Clone)]
pub struct AccountData {
    pub login: String,
    pub password: String,
    pub access_level: i32,
    pub online: i32,
    pub banned: i32,
    pub character_slot: i32,
    pub online_status: i32,
}

/// Load an account from the database by login name.
/// Uses CAST to handle INT UNSIGNED columns safely.
pub async fn load_account(pool: &MySqlPool, login: &str) -> Result<Option<AccountData>> {
    let row: Option<(String, String, i32, i32, i32, i32, i32)> = sqlx::query_as(
        "SELECT login, password, CAST(access_level AS SIGNED), CAST(online AS SIGNED), \
         CAST(banned AS SIGNED), CAST(character_slot AS SIGNED), CAST(OnlineStatus AS SIGNED) \
         FROM accounts WHERE login = ? LIMIT 1"
    )
    .bind(login)
    .fetch_optional(pool)
    .await?;

    Ok(row.map(|r| AccountData {
        login: r.0,
        password: r.1,
        access_level: r.2,
        online: r.3,
        banned: r.4,
        character_slot: r.5,
        online_status: r.6,
    }))
}

/// Validate a raw password against the stored SHA-1 + Base64 hash.
///
/// Algorithm (matching Java):
///   1. SHA-1 hash the raw password bytes
///   2. Base64 encode the hash
///   3. Compare with stored value
pub fn validate_password(raw_password: &str, stored_hash: &str) -> bool {
    let mut hasher = Sha1::new();
    hasher.update(raw_password.as_bytes());
    let hash = hasher.finalize();
    let encoded = base64::engine::general_purpose::STANDARD.encode(hash);
    encoded == stored_hash
}

/// Update account online status after successful login.
pub async fn set_online(pool: &MySqlPool, login: &str, ip: &str) -> Result<()> {
    sqlx::query("UPDATE accounts SET online = 1, ip = ?, lastactive = NOW() WHERE login = ?")
        .bind(ip)
        .bind(login)
        .execute(pool)
        .await?;
    Ok(())
}

/// Clear account online status on disconnect.
pub async fn set_offline(pool: &MySqlPool, login: &str) -> Result<()> {
    sqlx::query("UPDATE accounts SET online = 0, OnlineStatus = 0 WHERE login = ?")
        .bind(login)
        .execute(pool)
        .await?;
    Ok(())
}

/// Create a new account with SHA-1 + Base64 hashed password.
pub async fn create_account(
    pool: &MySqlPool,
    login: &str,
    raw_password: &str,
) -> Result<()> {
    let mut hasher = Sha1::new();
    hasher.update(raw_password.as_bytes());
    let hash = hasher.finalize();
    let encoded = base64::engine::general_purpose::STANDARD.encode(hash);

    sqlx::query("INSERT INTO accounts (login, password, access_level, online, banned, character_slot) VALUES (?, ?, 0, 0, 0, 0)")
        .bind(login)
        .bind(&encoded)
        .execute(pool)
        .await?;
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_password_hashing() {
        // Test that our hash matches Java's MessageDigest("SHA") + Base64
        let password = "test123";
        let mut hasher = Sha1::new();
        hasher.update(password.as_bytes());
        let hash = hasher.finalize();
        let encoded = base64::engine::general_purpose::STANDARD.encode(hash);

        assert!(validate_password(password, &encoded));
        assert!(!validate_password("wrong", &encoded));
    }
}
