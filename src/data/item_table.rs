use std::collections::HashMap;

use anyhow::Result;
use sqlx::{MySqlPool, Row};
use tracing::info;

use crate::ecs::components::item::{ItemTemplate, ItemType2};

/// Load all item templates from the three item tables.
pub async fn load_item_templates(pool: &MySqlPool) -> Result<HashMap<i32, ItemTemplate>> {
    let mut templates = HashMap::new();

    // Load etcitems
    let count = load_etcitems(pool, &mut templates).await?;
    info!("Loaded {} etcitems", count);

    // Load weapons
    let count = load_weapons(pool, &mut templates).await?;
    info!("Loaded {} weapons", count);

    // Load armors
    let count = load_armors(pool, &mut templates).await?;
    info!("Loaded {} armors", count);

    info!("Total item templates: {}", templates.len());
    Ok(templates)
}

async fn load_etcitems(pool: &MySqlPool, map: &mut HashMap<i32, ItemTemplate>) -> Result<usize> {
    let rows = sqlx::query(
        "SELECT item_id, name, unidentified_name_id, identified_name_id, \
         item_type, use_type, material, weight, invgfx, grdgfx, itemdesc_id, \
         dmg_small, dmg_large, min_lvl, max_lvl, bless, trade, cant_delete, \
         stackable, max_charge_count, food_volume, save_at_once \
         FROM etcitem"
    )
    .fetch_all(pool)
    .await?;

    let count = rows.len();
    for r in &rows {
        let item_id: i32 = r.get(0);
        let mut t = ItemTemplate::default();
        t.item_id = item_id;
        t.name = r.get(1);
        t.unidentified_name_id = r.get(2);
        t.identified_name_id = r.get(3);
        t.type2 = ItemType2::EtcItem;
        t.item_type = r.get(4);
        t.use_type = r.get(5);
        t.material = r.get(6);
        t.weight = r.get(7);
        t.inv_gfx_id = r.get(8);
        t.ground_gfx_id = r.get(9);
        t.item_desc_id = r.get(10);
        t.dmg_small = r.get(11);
        t.dmg_large = r.get(12);
        t.min_level = r.get(13);
        t.max_level = r.get(14);
        t.bless = r.get(15);
        t.tradable = r.get::<i32, _>(16) != 0;
        t.cant_delete = r.get::<i32, _>(17) != 0;
        t.stackable = r.get::<i32, _>(18) != 0;
        t.max_charge_count = r.get(19);
        t.food_volume = r.get(20);
        map.insert(item_id, t);
    }
    Ok(count)
}

async fn load_weapons(pool: &MySqlPool, map: &mut HashMap<i32, ItemTemplate>) -> Result<usize> {
    let rows = sqlx::query(
        "SELECT item_id, name, unidentified_name_id, identified_name_id, \
         type, material, weight, invgfx, grdgfx, itemdesc_id, \
         dmg_small, dmg_large, `range`, safenchant, \
         use_royal, use_knight, use_elf, use_mage, use_darkelf, use_dragonknight, use_illusionist, \
         hitmodifier, dmgmodifier, \
         add_str, add_dex, add_con, add_int, add_wis, add_cha, \
         add_hp, add_mp, add_hpr, add_mpr, add_sp, m_def, \
         double_dmg_chance, magicdmgmodifier, canbedmg, \
         min_lvl, max_lvl, bless, trade, cant_delete, haste_item, max_use_time \
         FROM weapon"
    )
    .fetch_all(pool)
    .await?;

    let count = rows.len();
    for r in &rows {
        let item_id: i32 = r.get(0);
        let mut t = ItemTemplate::default();
        t.item_id = item_id;
        t.name = r.get(1);
        t.unidentified_name_id = r.get(2);
        t.identified_name_id = r.get(3);
        t.type2 = ItemType2::Weapon;
        t.item_type = r.get(4);
        t.material = r.get(5);
        t.weight = r.get(6);
        t.inv_gfx_id = r.get(7);
        t.ground_gfx_id = r.get(8);
        t.item_desc_id = r.get(9);
        t.dmg_small = r.get(10);
        t.dmg_large = r.get(11);
        t.range = r.get(12);
        t.safe_enchant = r.get(13);
        t.use_royal = r.get::<i32, _>(14) != 0;
        t.use_knight = r.get::<i32, _>(15) != 0;
        t.use_elf = r.get::<i32, _>(16) != 0;
        t.use_mage = r.get::<i32, _>(17) != 0;
        t.use_darkelf = r.get::<i32, _>(18) != 0;
        t.use_dragonknight = r.get::<i32, _>(19) != 0;
        t.use_illusionist = r.get::<i32, _>(20) != 0;
        t.hit_modifier = r.get(21);
        t.dmg_modifier = r.get(22);
        t.add_str = r.get(23);
        t.add_dex = r.get(24);
        t.add_con = r.get(25);
        t.add_int = r.get(26);
        t.add_wis = r.get(27);
        t.add_cha = r.get(28);
        t.add_hp = r.get(29);
        t.add_mp = r.get(30);
        t.add_hpr = r.get(31);
        t.add_mpr = r.get(32);
        t.add_sp = r.get(33);
        t.m_def = r.get(34);
        t.double_dmg_chance = r.get(35);
        t.magic_dmg_modifier = r.get(36);
        t.min_level = r.get(38);
        t.max_level = r.get(39);
        t.bless = r.get(40);
        t.tradable = r.get::<i32, _>(41) != 0;
        t.cant_delete = r.get::<i32, _>(42) != 0;
        t.haste_item = r.get::<i32, _>(43) != 0;
        t.max_use_time = r.get(44);
        map.insert(item_id, t);
    }
    Ok(count)
}

async fn load_armors(pool: &MySqlPool, map: &mut HashMap<i32, ItemTemplate>) -> Result<usize> {
    let rows = sqlx::query(
        "SELECT item_id, name, unidentified_name_id, identified_name_id, \
         type, material, weight, invgfx, grdgfx, itemdesc_id, \
         ac, safenchant, \
         use_royal, use_knight, use_elf, use_mage, use_darkelf, use_dragonknight, use_illusionist, \
         add_str, add_dex, add_con, add_int, add_wis, add_cha, \
         add_hp, add_mp, add_hpr, add_mpr, add_sp, m_def, \
         damage_reduction, weight_reduction, \
         hit_modifier, dmg_modifier, bow_hit_modifier, bow_dmg_modifier, \
         haste_item, bless, trade, cant_delete, \
         min_lvl, max_lvl, max_use_time, grade \
         FROM armor"
    )
    .fetch_all(pool)
    .await?;

    let count = rows.len();
    for r in &rows {
        let item_id: i32 = r.get(0);
        let mut t = ItemTemplate::default();
        t.item_id = item_id;
        t.name = r.get(1);
        t.unidentified_name_id = r.get(2);
        t.identified_name_id = r.get(3);
        t.type2 = ItemType2::Armor;
        t.item_type = r.get(4);
        t.material = r.get(5);
        t.weight = r.get(6);
        t.inv_gfx_id = r.get(7);
        t.ground_gfx_id = r.get(8);
        t.item_desc_id = r.get(9);
        t.ac = r.get(10);
        t.safe_enchant = r.get(11);
        t.use_royal = r.get::<i32, _>(12) != 0;
        t.use_knight = r.get::<i32, _>(13) != 0;
        t.use_elf = r.get::<i32, _>(14) != 0;
        t.use_mage = r.get::<i32, _>(15) != 0;
        t.use_darkelf = r.get::<i32, _>(16) != 0;
        t.use_dragonknight = r.get::<i32, _>(17) != 0;
        t.use_illusionist = r.get::<i32, _>(18) != 0;
        t.add_str = r.get(19);
        t.add_dex = r.get(20);
        t.add_con = r.get(21);
        t.add_int = r.get(22);
        t.add_wis = r.get(23);
        t.add_cha = r.get(24);
        t.add_hp = r.get(25);
        t.add_mp = r.get(26);
        t.add_hpr = r.get(27);
        t.add_mpr = r.get(28);
        t.add_sp = r.get(29);
        t.m_def = r.get(30);
        t.damage_reduction = r.get(31);
        t.weight_reduction = r.get(32);
        t.hit_modifier = r.get(33);
        t.dmg_modifier = r.get(34);
        t.haste_item = r.get::<i32, _>(37) != 0;
        t.bless = r.get(38);
        t.tradable = r.get::<i32, _>(39) != 0;
        t.cant_delete = r.get::<i32, _>(40) != 0;
        t.min_level = r.get(41);
        t.max_level = r.get(42);
        t.max_use_time = r.get(43);
        map.insert(item_id, t);
    }
    Ok(count)
}
