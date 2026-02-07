/// Character creation DB operations.

use anyhow::Result;
use sqlx::MySqlPool;

use crate::protocol::client::char_create::{self, NewChar};

/// Create a new character in the database.
///
/// Returns the new objid, or an error string.
pub async fn create_character(
    pool: &MySqlPool,
    account_name: &str,
    nc: &NewChar,
    objid: i32,
) -> Result<i32> {
    let hp = char_create::get_init_hp(nc.char_type);
    let mp = char_create::calc_init_mp(nc.char_type, nc.wis_stat);
    let _gfxid = char_create::get_gfx_id(nc.char_type, nc.sex);

    // Birthday as yyyyMMdd integer
    let now = chrono_free_birthday();

    sqlx::query(
        "INSERT INTO characters SET \
         account_name=?, objid=?, char_name=?, birthday=?, level=1, HighLevel=1, \
         Exp=0, MaxHp=?, CurHp=?, MaxMp=?, CurMp=?, Ac=10, \
         Str=?, Con=?, Dex=?, Cha=?, Intel=?, Wis=?, \
         Status=0, Class=0, Sex=?, Type=?, Heading=0, \
         LocX=?, LocY=?, MapID=?, Food=40, Lawful=0, Title='', \
         ClanID=0, Clanname='', ClanRank=0, BonusStatus=0, ElixirStatus=0, \
         ElfAttr=0, PKcount=0, PkCountForElf=0, ExpRes=0, PartnerID=0, \
         AccessLevel=0, OnlineStatus=0, HomeTownID=0, Contribution=0, \
         Pay=0, HellTime=0, Banned=0, Karma=0, LastPk=NULL, LastPkForElf=NULL, \
         DeleteTime=NULL"
    )
    .bind(account_name)
    .bind(objid)
    .bind(&nc.name)
    .bind(now)
    .bind(hp)
    .bind(hp)
    .bind(mp)
    .bind(mp)
    .bind(nc.str_stat)
    .bind(nc.con_stat)
    .bind(nc.dex_stat)
    .bind(nc.cha_stat)
    .bind(nc.int_stat)
    .bind(nc.wis_stat)
    .bind(nc.sex)
    .bind(nc.char_type)
    .bind(char_create::START_X)
    .bind(char_create::START_Y)
    .bind(char_create::START_MAP)
    .execute(pool)
    .await?;

    Ok(objid)
}

/// Check if a character name already exists.
pub async fn name_exists(pool: &MySqlPool, name: &str) -> Result<bool> {
    let (count,): (i64,) = sqlx::query_as(
        "SELECT COUNT(*) FROM characters WHERE char_name = ?"
    )
    .bind(name)
    .fetch_one(pool)
    .await?;
    Ok(count > 0)
}

/// Generate a birthday integer in yyyyMMdd format without chrono crate.
fn chrono_free_birthday() -> i32 {
    // Use system time to get approximate date
    let secs = std::time::SystemTime::now()
        .duration_since(std::time::UNIX_EPOCH)
        .unwrap()
        .as_secs();
    // Rough calculation: days since epoch
    let days = secs / 86400;
    let years = 1970 + days / 365;
    let remaining = days % 365;
    let month = remaining / 30 + 1;
    let day = remaining % 30 + 1;
    (years as i32) * 10000 + (month as i32) * 100 + (day as i32)
}
