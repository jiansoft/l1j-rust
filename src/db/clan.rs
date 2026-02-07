/// Clan database operations.
///
/// Ported from Java ClanTable.java + ClanMembersTable.java.
/// Full CRUD for clan_data and clan_members tables.

use anyhow::Result;
use sqlx::{MySqlPool, Row};
use tracing::info;

/// Clan data loaded from `clan_data` table.
#[derive(Debug, Clone)]
pub struct ClanRow {
    pub clan_id: i32,
    pub clan_name: String,
    pub leader_id: i32,
    pub leader_name: String,
    pub has_castle: i32,
    pub has_house: i32,
    pub announcement: String,
    pub emblem_id: i32,
    pub emblem_status: i32,
}

/// Clan member row from `clan_members` table.
#[derive(Debug, Clone)]
pub struct ClanMemberRow {
    pub clan_id: i32,
    pub index_id: i32,
    pub char_id: i32,
    pub char_name: String,
    pub notes: String,
}

// ---------------------------------------------------------------------------
// clan_data CRUD
// ---------------------------------------------------------------------------

/// Load all clans from the database (called at server startup).
pub async fn load_all_clans(pool: &MySqlPool) -> Result<Vec<ClanRow>> {
    let rows = sqlx::query(
        "SELECT clan_id, clan_name, leader_id, leader_name, hascastle, hashouse, \
         IFNULL(announcement,''), IFNULL(emblem_id,0), IFNULL(emblem_status,0) \
         FROM clan_data ORDER BY clan_id",
    )
    .fetch_all(pool)
    .await?;

    let clans: Vec<ClanRow> = rows
        .iter()
        .map(|r| ClanRow {
            clan_id: r.get(0),
            clan_name: r.get(1),
            leader_id: r.get(2),
            leader_name: r.get(3),
            has_castle: r.get(4),
            has_house: r.get(5),
            announcement: r.get(6),
            emblem_id: r.get(7),
            emblem_status: r.get(8),
        })
        .collect();

    info!("Loaded {} clans", clans.len());
    Ok(clans)
}

/// Create a new clan.
pub async fn create_clan(
    pool: &MySqlPool,
    clan_id: i32,
    clan_name: &str,
    leader_id: i32,
    leader_name: &str,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO clan_data SET clan_id=?, clan_name=?, leader_id=?, leader_name=?, \
         hascastle=0, hashouse=0, found_date=NOW(), announcement='', emblem_id=0, emblem_status=0",
    )
    .bind(clan_id)
    .bind(clan_name)
    .bind(leader_id)
    .bind(leader_name)
    .execute(pool)
    .await?;

    info!("Created clan: {} (id={})", clan_name, clan_id);
    Ok(())
}

/// Update clan data.
pub async fn update_clan(pool: &MySqlPool, clan: &ClanRow) -> Result<()> {
    sqlx::query(
        "UPDATE clan_data SET leader_id=?, leader_name=?, hascastle=?, hashouse=?, \
         announcement=?, emblem_id=?, emblem_status=? WHERE clan_id=?",
    )
    .bind(clan.leader_id)
    .bind(&clan.leader_name)
    .bind(clan.has_castle)
    .bind(clan.has_house)
    .bind(&clan.announcement)
    .bind(clan.emblem_id)
    .bind(clan.emblem_status)
    .bind(clan.clan_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Delete a clan by name.
pub async fn delete_clan(pool: &MySqlPool, clan_name: &str) -> Result<()> {
    sqlx::query("DELETE FROM clan_data WHERE clan_name=?")
        .bind(clan_name)
        .execute(pool)
        .await?;

    info!("Deleted clan: {}", clan_name);
    Ok(())
}

// ---------------------------------------------------------------------------
// clan_members CRUD
// ---------------------------------------------------------------------------

/// Load all members for a clan.
pub async fn load_clan_members(pool: &MySqlPool, clan_id: i32) -> Result<Vec<ClanMemberRow>> {
    let rows = sqlx::query(
        "SELECT clan_id, index_id, char_id, char_name, IFNULL(notes,'') \
         FROM clan_members WHERE clan_id = ?",
    )
    .bind(clan_id)
    .fetch_all(pool)
    .await?;

    Ok(rows
        .iter()
        .map(|r| ClanMemberRow {
            clan_id: r.get(0),
            index_id: r.get(1),
            char_id: r.get(2),
            char_name: r.get(3),
            notes: r.get(4),
        })
        .collect())
}

/// Add a new clan member.
pub async fn add_member(
    pool: &MySqlPool,
    clan_id: i32,
    index_id: i32,
    char_id: i32,
    char_name: &str,
) -> Result<()> {
    sqlx::query(
        "INSERT INTO clan_members SET clan_id=?, index_id=?, char_id=?, char_name=?, notes=''",
    )
    .bind(clan_id)
    .bind(index_id)
    .bind(char_id)
    .bind(char_name)
    .execute(pool)
    .await?;
    Ok(())
}

/// Remove a clan member by character ID.
pub async fn delete_member(pool: &MySqlPool, char_id: i32) -> Result<()> {
    sqlx::query("DELETE FROM clan_members WHERE char_id=?")
        .bind(char_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Remove ALL members of a clan (used when disbanding).
pub async fn delete_all_members(pool: &MySqlPool, clan_id: i32) -> Result<()> {
    sqlx::query("DELETE FROM clan_members WHERE clan_id=?")
        .bind(clan_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Update member notes.
pub async fn update_member_notes(pool: &MySqlPool, char_id: i32, notes: &str) -> Result<()> {
    sqlx::query("UPDATE clan_members SET notes=? WHERE char_id=?")
        .bind(notes)
        .bind(char_id)
        .execute(pool)
        .await?;
    Ok(())
}

/// Clear clan ID from the characters table when a player leaves/is kicked.
pub async fn clear_character_clan(pool: &MySqlPool, char_id: i32) -> Result<()> {
    sqlx::query(
        "UPDATE characters SET ClanID=0, Clanname='', ClanRank=0, ClanMemberId=0 WHERE objid=?",
    )
    .bind(char_id)
    .execute(pool)
    .await?;
    Ok(())
}

/// Set clan ID on the characters table when a player joins.
pub async fn set_character_clan(
    pool: &MySqlPool,
    char_id: i32,
    clan_id: i32,
    clan_name: &str,
    rank: i32,
    member_id: i32,
) -> Result<()> {
    sqlx::query(
        "UPDATE characters SET ClanID=?, Clanname=?, ClanRank=?, ClanMemberId=? WHERE objid=?",
    )
    .bind(clan_id)
    .bind(clan_name)
    .bind(rank)
    .bind(member_id)
    .bind(char_id)
    .execute(pool)
    .await?;
    Ok(())
}
