use std::collections::HashMap;

use anyhow::Result;
use sqlx::{MySqlPool, Row};
use tracing::info;

use crate::ecs::components::skill::SkillTemplate;

/// Load all skill templates from the `skills` database table.
pub async fn load_skill_templates(pool: &MySqlPool) -> Result<HashMap<i32, SkillTemplate>> {
    let rows = sqlx::query(
        "SELECT skill_id, name, skill_level, skill_number, \
         mpConsume, hpConsume, itemConsumeId, itemConsumeCount, \
         reuseDelay, buffDuration, target, target_to, \
         damage_value, damage_dice, damage_dice_count, probability_value, \
         attr, type as skill_type, isThrough, \
         ranged, area, actid, castgfx, castgfx2, \
         sysmsgID_happen, sysmsgID_stop, sysmsgID_fail \
         FROM skills"
    )
    .fetch_all(pool)
    .await?;

    let mut templates = HashMap::with_capacity(rows.len());

    for r in &rows {
        let skill_id: i32 = r.get(0);
        templates.insert(skill_id, SkillTemplate {
            skill_id,
            name: r.get(1),
            skill_level: r.get(2),
            skill_number: r.get(3),
            mp_consume: r.get(4),
            hp_consume: r.get(5),
            item_consume_id: r.get(6),
            item_consume_count: r.get(7),
            reuse_delay: r.get(8),
            buff_duration: r.get(9),
            target: r.get(10),
            target_to: r.get(11),
            damage_value: r.get(12),
            damage_dice: r.get(13),
            damage_dice_count: r.get(14),
            probability_value: r.get(15),
            attr: r.get(16),
            skill_type: r.get(17),
            is_through: r.get::<i32, _>(18) != 0,
            range: r.get(19),
            area: r.get(20),
            action_id: r.get(21),
            cast_gfx: r.get(22),
            cast_gfx2: r.get(23),
            sys_msg_id_happen: r.get(24),
            sys_msg_id_stop: r.get(25),
            sys_msg_id_fail: r.get(26),
        });
    }

    info!("Loaded {} skill templates", templates.len());
    Ok(templates)
}
