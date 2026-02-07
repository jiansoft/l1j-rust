/// Character teleport bookmarks (記憶座標).
///
/// Ported from Java L1BookMark.java. Stores saved locations
/// that players can teleport to.

use anyhow::Result;
use sqlx::{MySqlPool, Row};

/// A single bookmark entry.
#[derive(Debug, Clone)]
pub struct Bookmark {
    pub id: i32,
    pub char_id: i32,
    pub name: String,
    pub loc_x: i32,
    pub loc_y: i32,
    pub map_id: i32,
}

/// Maximum bookmarks per character.
pub const MAX_BOOKMARKS: usize = 49;

/// Load all bookmarks for a character.
pub async fn load_bookmarks(pool: &MySqlPool, char_id: i32) -> Result<Vec<Bookmark>> {
    let rows = sqlx::query(
        "SELECT id, char_id, name, locx, locy, mapid FROM character_teleport WHERE char_id = ?",
    )
    .bind(char_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .iter()
        .map(|r| Bookmark {
            id: r.get(0),
            char_id: r.get(1),
            name: r.get(2),
            loc_x: r.get(3),
            loc_y: r.get(4),
            map_id: r.get(5),
        })
        .collect())
}

/// Add a new bookmark.
pub async fn add_bookmark(
    pool: &MySqlPool,
    id: i32,
    char_id: i32,
    name: &str,
    x: i32,
    y: i32,
    map_id: i32,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO character_teleport SET id = ?, char_id = ?, name = ?, locx = ?, locy = ?, mapid = ?",
    )
    .bind(id)
    .bind(char_id)
    .bind(name)
    .bind(x)
    .bind(y)
    .bind(map_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete a bookmark by ID.
pub async fn delete_bookmark(pool: &MySqlPool, bookmark_id: i32) -> Result<()> {
    sqlx::query("DELETE FROM character_teleport WHERE id = ?")
        .bind(bookmark_id)
        .execute(pool)
        .await?;
    Ok(())
}
