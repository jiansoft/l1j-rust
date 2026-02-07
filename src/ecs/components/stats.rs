/// Health and mana component.
#[derive(Debug, Clone)]
pub struct Health {
    pub cur_hp: i32,
    pub max_hp: i32,
    pub cur_mp: i32,
    pub max_mp: i32,
}

/// Combat stats component.
#[derive(Debug, Clone)]
pub struct CombatStats {
    pub level: i32,
    pub str_stat: i32,
    pub dex_stat: i32,
    pub con_stat: i32,
    pub wis_stat: i32,
    pub cha_stat: i32,
    pub int_stat: i32,
    pub ac: i32,
    pub mr: i32,
    pub exp: i32,
    pub lawful: i32,
}

/// Speed values for animation timing.
#[derive(Debug, Clone)]
pub struct Speed {
    pub move_speed: i32,     // passispeed
    pub atk_speed: i32,      // atkspeed
    pub atk_magic_speed: i32,
    pub brave_speed: i32,
}
