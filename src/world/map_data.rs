/// Map tile data and passability system.
///
/// Supports L1J V1 (text) and V2 (binary compressed) map formats.
/// Each tile stores passability flags, zone type, and arrow passability.

/// V1 tile bit flags (from L1V1Map.java).
pub mod v1_flags {
    pub const PASSABLE_EAST: u8 = 0x01;
    pub const PASSABLE_NORTH: u8 = 0x02;
    pub const ARROW_PASSABLE_EAST: u8 = 0x04;
    pub const ARROW_PASSABLE_NORTH: u8 = 0x08;
    pub const ZONE_SAFETY: u8 = 0x10;
    pub const ZONE_COMBAT: u8 = 0x20;
    pub const IMPASSABLE: u8 = 0x80;
}

/// Known V2 impassable tile values.
const V2_IMPASSABLE: &[u16] = &[1, 9, 65, 69, 73];

/// Map properties from the `mapids` table.
#[derive(Debug, Clone)]
pub struct MapProperties {
    pub map_id: i32,
    pub locationname: String,
    pub startx: i32,
    pub endx: i32,
    pub starty: i32,
    pub endy: i32,
    pub monster_amount: f64,
    pub drop_rate: f64,
    pub underwater: bool,
    pub markable: bool,
    pub teleportable: bool,
    pub escapable: bool,
    pub resurrection: bool,
    pub painwand: bool,
    pub death_penalty: bool,
    pub take_pets: bool,
    pub recall_pets: bool,
    pub usable_item: bool,
    pub usable_skill: bool,
}

impl Default for MapProperties {
    fn default() -> Self {
        MapProperties {
            map_id: 0,
            locationname: String::new(),
            startx: 0,
            endx: 0,
            starty: 0,
            endy: 0,
            monster_amount: 1.0,
            drop_rate: 1.0,
            underwater: false,
            markable: true,
            teleportable: true,
            escapable: true,
            resurrection: true,
            painwand: true,
            death_penalty: true,
            take_pets: true,
            recall_pets: true,
            usable_item: true,
            usable_skill: true,
        }
    }
}

/// A loaded map's tile data.
///
/// Supports both V1 (byte per tile) and V2 (u16 per tile) formats.
/// Once loaded, the format difference is abstracted away - callers
/// use `is_passable()`, `is_safety_zone()`, etc.
#[derive(Debug, Clone)]
pub struct MapTileData {
    pub map_id: i32,
    pub x_loc: i32,  // world X of top-left corner
    pub y_loc: i32,  // world Y of top-left corner
    pub width: i32,
    pub height: i32,
    tiles_v1: Option<Vec<u8>>,   // V1: 1 byte per tile
    tiles_v2: Option<Vec<u16>>,  // V2: 1 u16 per tile
    pub props: MapProperties,
}

impl MapTileData {
    /// Create a V1 map from raw tile bytes.
    pub fn from_v1(
        map_id: i32,
        x_loc: i32,
        y_loc: i32,
        width: i32,
        height: i32,
        tiles: Vec<u8>,
        props: MapProperties,
    ) -> Self {
        MapTileData {
            map_id,
            x_loc,
            y_loc,
            width,
            height,
            tiles_v1: Some(tiles),
            tiles_v2: None,
            props,
        }
    }

    /// Create a V2 map from raw tile u16 values.
    pub fn from_v2(
        map_id: i32,
        x_loc: i32,
        y_loc: i32,
        width: i32,
        height: i32,
        tiles: Vec<u16>,
        props: MapProperties,
    ) -> Self {
        MapTileData {
            map_id,
            x_loc,
            y_loc,
            width,
            height,
            tiles_v1: None,
            tiles_v2: Some(tiles),
            props,
        }
    }

    /// Create a null/empty map (returned when map not found).
    pub fn null_map(map_id: i32) -> Self {
        MapTileData {
            map_id,
            x_loc: 0,
            y_loc: 0,
            width: 0,
            height: 0,
            tiles_v1: None,
            tiles_v2: None,
            props: MapProperties::default(),
        }
    }

    /// Check if a world coordinate is within this map's bounds.
    pub fn in_bounds(&self, x: i32, y: i32) -> bool {
        x >= self.x_loc
            && x < self.x_loc + self.width
            && y >= self.y_loc
            && y < self.y_loc + self.height
    }

    /// Get the raw V1 tile byte at world coordinates.
    fn get_v1_tile(&self, x: i32, y: i32) -> u8 {
        if !self.in_bounds(x, y) {
            return v1_flags::IMPASSABLE;
        }
        if let Some(ref tiles) = self.tiles_v1 {
            let idx = ((y - self.y_loc) * self.width + (x - self.x_loc)) as usize;
            if idx < tiles.len() {
                return tiles[idx];
            }
        }
        v1_flags::IMPASSABLE
    }

    /// Get the raw V2 tile value at world coordinates.
    fn get_v2_tile(&self, x: i32, y: i32) -> u16 {
        if !self.in_bounds(x, y) {
            return 1; // impassable
        }
        if let Some(ref tiles) = self.tiles_v2 {
            let idx = ((y - self.y_loc) * self.width + (x - self.x_loc)) as usize;
            if idx < tiles.len() {
                return tiles[idx];
            }
        }
        1
    }

    /// Check if a tile is passable (for walking).
    pub fn is_passable(&self, x: i32, y: i32) -> bool {
        if self.tiles_v1.is_some() {
            let tile = self.get_v1_tile(x, y);
            tile & v1_flags::IMPASSABLE == 0
        } else if self.tiles_v2.is_some() {
            let tile = self.get_v2_tile(x, y);
            !V2_IMPASSABLE.contains(&tile)
        } else {
            false // null map
        }
    }

    /// Check if a tile is a safety zone.
    pub fn is_safety_zone(&self, x: i32, y: i32) -> bool {
        if self.tiles_v1.is_some() {
            let tile = self.get_v1_tile(x, y);
            tile & v1_flags::ZONE_SAFETY != 0
        } else if self.tiles_v2.is_some() {
            self.get_v2_tile(x, y) == 4
        } else {
            false
        }
    }

    /// Check if a tile is a combat zone.
    pub fn is_combat_zone(&self, x: i32, y: i32) -> bool {
        if self.tiles_v1.is_some() {
            let tile = self.get_v1_tile(x, y);
            tile & v1_flags::ZONE_COMBAT != 0
        } else if self.tiles_v2.is_some() {
            self.get_v2_tile(x, y) == 8
        } else {
            false
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_v1_passability() {
        let tiles = vec![0x00, 0x80, 0x10, 0x03];
        let map = MapTileData::from_v1(0, 100, 200, 2, 2, tiles, MapProperties::default());

        assert!(map.is_passable(100, 200));   // 0x00 - passable
        assert!(!map.is_passable(101, 200));  // 0x80 - impassable
        assert!(map.is_passable(100, 201));   // 0x10 - passable + safety
        assert!(map.is_safety_zone(100, 201));
        assert!(!map.is_passable(99, 199));   // out of bounds
    }

    #[test]
    fn test_v2_passability() {
        let tiles = vec![0, 1, 4, 8];
        let map = MapTileData::from_v2(0, 100, 200, 2, 2, tiles, MapProperties::default());

        assert!(map.is_passable(100, 200));   // 0 - passable
        assert!(!map.is_passable(101, 200));  // 1 - impassable
        assert!(map.is_safety_zone(100, 201)); // 4 - safety
        assert!(map.is_combat_zone(101, 201)); // 8 - combat
    }

    #[test]
    fn test_null_map() {
        let map = MapTileData::null_map(999);
        assert!(!map.is_passable(0, 0));
    }
}
