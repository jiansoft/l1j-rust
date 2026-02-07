use std::collections::HashMap;

use anyhow::Result;
use sqlx::{MySqlPool, Row};
use tracing::info;

use crate::ecs::components::npc::NpcTemplate;

/// Load all NPC templates from the `npc` database table.
///
/// Returns a HashMap keyed by npc_id for fast template lookup.
pub async fn load_npc_templates(pool: &MySqlPool) -> Result<HashMap<i32, NpcTemplate>> {
    let rows = sqlx::query(
        "SELECT npcid, name, nameid, impl, gfxid, lvl, hp, mp, ac, \
         str, con, dex, wis, intel, mr, exp, lawful, size, \
         undead, poison_atk, paralysis_atk, agro, agrososc, agrocoi, \
         family, agrofamily, picupitem, bravespeed, passispeed, atkspeed, \
         atk_magic_speed, tamable, teleport, doppel, \
         hprinterval, hpr, mprinterval, mpr, ranged, light_size, \
         change_head, damage_reduction, hard, karma, \
         transform_id, transform_gfxid, cant_resurrect \
         FROM npc",
    )
    .fetch_all(pool)
    .await?;

    let mut templates = HashMap::with_capacity(rows.len());

    for r in &rows {
        let npc_id: i32 = r.get(0);
        let template = NpcTemplate {
            npc_id,
            name: r.get(1),
            nameid: r.get(2),
            impl_type: r.get(3),
            gfxid: r.get(4),
            level: r.get(5),
            hp: r.get(6),
            mp: r.get(7),
            ac: r.get(8),
            str_stat: r.get(9),
            con_stat: r.get(10),
            dex_stat: r.get(11),
            wis_stat: r.get(12),
            int_stat: r.get(13),
            mr: r.get(14),
            exp: r.get(15),
            lawful: r.get(16),
            size: r.get::<String, _>(17),
            undead: r.get(18),
            poison_atk: r.get(19),
            paralysis_atk: r.get(20),
            agro: r.get::<i32, _>(21) != 0,
            agrososc: r.get::<i32, _>(22) != 0,
            agrocoi: r.get::<i32, _>(23) != 0,
            family: r.get(24),
            agrofamily: r.get(25),
            pickup_item: r.get::<i32, _>(26) != 0,
            brave_speed: r.get(27),
            passispeed: r.get(28),
            atkspeed: r.get(29),
            atk_magic_speed: r.get(30),
            tamable: r.get::<i32, _>(31) != 0,
            teleport: r.get::<i32, _>(32) != 0,
            doppel: r.get::<i32, _>(33) != 0,
            hpr_interval: r.get(34),
            hpr: r.get(35),
            mpr_interval: r.get(36),
            mpr: r.get(37),
            ranged: r.get(38),
            light_size: r.get(39),
            change_head: r.get::<i32, _>(40) != 0,
            damage_reduction: r.get(41),
            hard: r.get::<i32, _>(42) != 0,
            karma: r.get(43),
            transform_id: r.get(44),
            transform_gfxid: r.get(45),
            cant_resurrect: r.get::<i32, _>(46) != 0,
        };
        templates.insert(npc_id, template);
    }

    info!("Loaded {} NPC templates", templates.len());
    Ok(templates)
}
