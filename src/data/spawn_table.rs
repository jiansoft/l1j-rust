use anyhow::Result;
use sqlx::{MySqlPool, Row};
use tracing::info;

use crate::ecs::components::npc::SpawnInfo;

/// Load all monster spawn data from the `spawnlist` database table.
///
/// Returns a Vec of SpawnInfo, each representing a spawn point.
pub async fn load_spawn_table(pool: &MySqlPool) -> Result<Vec<SpawnInfo>> {
    let rows = sqlx::query(
        "SELECT id, count, npc_templateid, locx, locy, mapid, heading, \
         randomx, randomy, min_respawn_delay, max_respawn_delay, \
         movement_distance \
         FROM spawnlist",
    )
    .fetch_all(pool)
    .await?;

    let mut spawns = Vec::with_capacity(rows.len());

    for r in &rows {
        spawns.push(SpawnInfo {
            spawn_id: r.get(0),
            count: r.get(1),
            npc_template_id: r.get(2),
            loc_x: r.get(3),
            loc_y: r.get(4),
            map_id: r.get::<i16, _>(5) as i32,
            heading: r.get(6),
            randomx: r.get(7),
            randomy: r.get(8),
            min_respawn_delay: r.get(9),
            max_respawn_delay: r.get(10),
            movement_distance: r.get(11),
        });
    }

    info!("Loaded {} spawn entries", spawns.len());
    Ok(spawns)
}

/// Load NPC spawn data from the `spawnlist_npc` database table.
pub async fn load_npc_spawn_table(pool: &MySqlPool) -> Result<Vec<SpawnInfo>> {
    let rows = sqlx::query(
        "SELECT id, count, npc_templateid, locx, locy, mapid, heading, \
         randomx, randomy, min_respawn_delay, max_respawn_delay, \
         movement_distance \
         FROM spawnlist_npc",
    )
    .fetch_all(pool)
    .await?;

    let mut spawns = Vec::with_capacity(rows.len());

    for r in &rows {
        spawns.push(SpawnInfo {
            spawn_id: r.get(0),
            count: r.get(1),
            npc_template_id: r.get(2),
            loc_x: r.get(3),
            loc_y: r.get(4),
            map_id: r.get::<i16, _>(5) as i32,
            heading: r.get(6),
            randomx: r.get(7),
            randomy: r.get(8),
            min_respawn_delay: r.get(9),
            max_respawn_delay: r.get(10),
            movement_distance: r.get(11),
        });
    }

    info!("Loaded {} NPC spawn entries", spawns.len());
    Ok(spawns)
}
