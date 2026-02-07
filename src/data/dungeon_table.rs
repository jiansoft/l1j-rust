/// Dungeon/portal entrance data loaded from the `dungeon` MySQL table.
///
/// Ported from Java Dungeon.java. Maps source coordinates to
/// destination coordinates for portal teleportation.

use std::collections::HashMap;

use anyhow::Result;
use sqlx::{MySqlPool, Row};
use tracing::info;

/// A single dungeon portal entry.
#[derive(Debug, Clone)]
pub struct DungeonEntry {
    pub src_map_id: i32,
    pub src_x: i32,
    pub src_y: i32,
    pub new_x: i32,
    pub new_y: i32,
    pub new_map_id: i32,
    pub new_heading: i32,
}

/// Dungeon portal lookup table.
/// Key format: "{map_id}_{x}_{y}" (matches Java's `srcMapId + srcX + srcY`)
pub struct DungeonTable {
    portals: HashMap<String, DungeonEntry>,
}

impl DungeonTable {
    /// Build the portal key from source coordinates.
    fn make_key(map_id: i32, x: i32, y: i32) -> String {
        format!("{}_{}", map_id, x * 10000 + y)
    }

    /// Load all portals from the database.
    pub async fn load(pool: &MySqlPool) -> Result<Self> {
        let rows = sqlx::query(
            "SELECT src_mapid, src_x, src_y, new_x, new_y, new_mapid, new_heading FROM dungeon",
        )
        .fetch_all(pool)
        .await?;

        let mut portals = HashMap::with_capacity(rows.len());

        for r in &rows {
            let entry = DungeonEntry {
                src_map_id: r.get(0),
                src_x: r.get(1),
                src_y: r.get(2),
                new_x: r.get(3),
                new_y: r.get(4),
                new_map_id: r.get(5),
                new_heading: r.get(6),
            };
            let key = Self::make_key(entry.src_map_id, entry.src_x, entry.src_y);
            portals.insert(key, entry);
        }

        info!("Loaded {} dungeon portals", portals.len());
        Ok(DungeonTable { portals })
    }

    /// Look up a portal at the given source coordinates.
    pub fn find_portal(&self, map_id: i32, x: i32, y: i32) -> Option<&DungeonEntry> {
        let key = Self::make_key(map_id, x, y);
        self.portals.get(&key)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_portal_key() {
        let key = DungeonTable::make_key(4, 32755, 32831);
        assert!(!key.is_empty());
    }

    #[test]
    fn test_portal_lookup_miss() {
        let table = DungeonTable {
            portals: HashMap::new(),
        };
        assert!(table.find_portal(4, 100, 200).is_none());
    }
}
