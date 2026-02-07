/// Shared game state accessible by all sessions.
///
/// This is the bridge that lets players see each other.
/// Each session registers itself here when entering the game,
/// and queries other players for visibility.

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::Mutex;

/// A connected player visible in the game world.
#[derive(Debug, Clone)]
pub struct OnlinePlayer {
    pub object_id: i32,
    pub name: String,
    pub x: i32,
    pub y: i32,
    pub map_id: i32,
    pub heading: i32,
    pub gfx_id: i32,
    pub level: i32,
    pub lawful: i32,
    pub char_type: i32,
    pub sex: i32,
    pub clan_name: String,
    pub title: String,
    /// Channel to send packets to this player's session.
    pub packet_tx: tokio::sync::mpsc::UnboundedSender<Vec<u8>>,
}

/// Shared state wrapped in Arc<Mutex> for cross-session access.
pub type SharedWorld = Arc<Mutex<WorldState>>;

pub struct WorldState {
    /// All online players keyed by object_id.
    pub players: HashMap<i32, OnlinePlayer>,
}

impl WorldState {
    pub fn new() -> Self {
        WorldState {
            players: HashMap::new(),
        }
    }

    /// Register a player when they enter the game.
    pub fn add_player(&mut self, player: OnlinePlayer) {
        self.players.insert(player.object_id, player);
    }

    /// Remove a player when they leave.
    pub fn remove_player(&mut self, object_id: i32) {
        self.players.remove(&object_id);
    }

    /// Update a player's position after movement.
    pub fn update_position(&mut self, object_id: i32, x: i32, y: i32, heading: i32) {
        if let Some(p) = self.players.get_mut(&object_id) {
            p.x = x;
            p.y = y;
            p.heading = heading;
        }
    }

    /// Get all players on the same map within screen range (18 tiles).
    pub fn get_nearby_players(&self, map_id: i32, x: i32, y: i32, exclude_id: i32) -> Vec<OnlinePlayer> {
        self.players.values()
            .filter(|p| {
                p.object_id != exclude_id
                    && p.map_id == map_id
                    && (p.x - x).abs() <= 18
                    && (p.y - y).abs() <= 18
            })
            .cloned()
            .collect()
    }

    /// Send a packet to all nearby players (broadcast).
    pub fn broadcast_to_nearby(&self, map_id: i32, x: i32, y: i32, exclude_id: i32, packet: &[u8]) {
        for p in self.players.values() {
            if p.object_id != exclude_id
                && p.map_id == map_id
                && (p.x - x).abs() <= 18
                && (p.y - y).abs() <= 18
            {
                let _ = p.packet_tx.send(packet.to_vec());
            }
        }
    }
}

pub fn create_shared_world() -> SharedWorld {
    Arc::new(Mutex::new(WorldState::new()))
}
