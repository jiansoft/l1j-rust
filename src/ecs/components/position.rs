/// Position component - every entity in the world has a position.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Position {
    pub x: i32,
    pub y: i32,
    pub map_id: i32,
    pub heading: i32,  // 0-7 direction (like L1J heading)
}

impl Position {
    pub fn new(x: i32, y: i32, map_id: i32) -> Self {
        Position { x, y, map_id, heading: 0 }
    }

    /// Check if another position is within screen range (18 tiles).
    pub fn is_in_screen(&self, other: &Position) -> bool {
        if self.map_id != other.map_id {
            return false;
        }
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        dx <= 18 && dy <= 18
    }

    /// Tile-line distance (Chebyshev distance).
    pub fn tile_distance(&self, other: &Position) -> i32 {
        if self.map_id != other.map_id {
            return i32::MAX;
        }
        let dx = (self.x - other.x).abs();
        let dy = (self.y - other.y).abs();
        dx.max(dy)
    }

    /// Move one step in the given heading direction.
    /// Heading: 0=S, 1=SW, 2=W, 3=NW, 4=N, 5=NE, 6=E, 7=SE
    pub fn step(&self, heading: i32) -> Position {
        let (dx, dy) = heading_delta(heading);
        Position {
            x: self.x + dx,
            y: self.y + dy,
            map_id: self.map_id,
            heading,
        }
    }
}

/// Get the (dx, dy) delta for a heading direction.
/// L1J heading: 0=South(+y), 1=SW, 2=West(-x), 3=NW, 4=North(-y), 5=NE, 6=East(+x), 7=SE
pub fn heading_delta(heading: i32) -> (i32, i32) {
    match heading & 7 {
        0 => (0, 1),    // South
        1 => (-1, 1),   // Southwest
        2 => (-1, 0),   // West
        3 => (-1, -1),  // Northwest
        4 => (0, -1),   // North
        5 => (1, -1),   // Northeast
        6 => (1, 0),    // East
        7 => (1, 1),    // Southeast
        _ => (0, 0),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_in_screen() {
        let a = Position::new(32800, 32800, 4);
        let b = Position::new(32810, 32810, 4);
        assert!(a.is_in_screen(&b));

        let c = Position::new(32830, 32830, 4);
        assert!(!a.is_in_screen(&c)); // > 18 tiles away
    }

    #[test]
    fn test_different_map() {
        let a = Position::new(100, 100, 4);
        let b = Position::new(100, 100, 5);
        assert!(!a.is_in_screen(&b));
        assert_eq!(a.tile_distance(&b), i32::MAX);
    }

    #[test]
    fn test_step() {
        let pos = Position::new(100, 100, 4);
        let moved = pos.step(6); // East
        assert_eq!(moved.x, 101);
        assert_eq!(moved.y, 100);
    }
}
