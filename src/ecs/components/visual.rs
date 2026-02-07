/// Visual appearance component - how an entity looks to clients.
#[derive(Debug, Clone)]
pub struct Visual {
    pub gfx_id: i32,       // sprite graphics ID
    pub temp_gfx_id: i32,  // temporary GFX (polymorph etc.)
    pub light_size: i32,
    pub name: String,
    pub nameid: String,
    pub title: String,
    pub status: i32,        // animation status
}

impl Visual {
    pub fn new_npc(gfx_id: i32, name: String, nameid: String) -> Self {
        Visual {
            gfx_id,
            temp_gfx_id: 0,
            light_size: 0,
            name,
            nameid,
            title: String::new(),
            status: 0,
        }
    }

    /// Get the effective GFX ID (temp if set, otherwise base).
    pub fn effective_gfx(&self) -> i32 {
        if self.temp_gfx_id != 0 {
            self.temp_gfx_id
        } else {
            self.gfx_id
        }
    }
}
