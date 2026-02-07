/// Clan (blood pledge) component - full implementation.
///
/// Ported from Java L1Clan.java. Includes all rank constants,
/// member management, and clan state.

/// Clan rank constants (from Java L1Clan).
pub mod ranks {
    pub const CLAN_RANK_LEAGUE_PUBLIC: i32 = 2;
    pub const CLAN_RANK_LEAGUE_VICEPRINCE: i32 = 3;
    pub const CLAN_RANK_LEAGUE_PRINCE: i32 = 4;
    pub const CLAN_RANK_LEAGUE_PROBATION: i32 = 5;
    pub const CLAN_RANK_LEAGUE_GUARDIAN: i32 = 6;
    pub const CLAN_RANK_PUBLIC: i32 = 7;
    pub const CLAN_RANK_PROBATION: i32 = 8;
    pub const CLAN_RANK_GUARDIAN: i32 = 9;
    pub const CLAN_RANK_PRINCE: i32 = 10;
}

/// Full clan data.
#[derive(Debug, Clone)]
pub struct ClanData {
    pub clan_id: i32,
    pub clan_name: String,
    pub leader_id: i32,
    pub leader_name: String,
    pub castle_id: i32,
    pub house_id: i32,
    pub emblem_id: i32,
    pub emblem_status: i32,
    pub announcement: String,
    pub member_names: Vec<String>,
    /// Character ID currently using clan warehouse (0 = nobody).
    pub warehouse_using_char: i32,
}

impl ClanData {
    pub fn new(clan_id: i32, clan_name: String, leader_id: i32, leader_name: String) -> Self {
        ClanData {
            clan_id,
            clan_name,
            leader_id,
            leader_name,
            castle_id: 0,
            house_id: 0,
            emblem_id: 0,
            emblem_status: 0,
            announcement: String::new(),
            member_names: Vec::new(),
            warehouse_using_char: 0,
        }
    }

    pub fn add_member(&mut self, name: String) {
        if !self.member_names.contains(&name) {
            self.member_names.push(name);
        }
    }

    pub fn remove_member(&mut self, name: &str) {
        self.member_names.retain(|n| n != name);
    }

    pub fn member_count(&self) -> usize {
        self.member_names.len()
    }

    pub fn has_castle(&self) -> bool {
        self.castle_id > 0
    }

    pub fn has_house(&self) -> bool {
        self.house_id > 0
    }
}

/// A player's clan membership component.
#[derive(Debug, Clone)]
pub struct ClanMembership {
    pub clan_id: i32,
    pub clan_name: String,
    pub rank: i32,
    pub member_id: i32,     // index_id from clan_members table
    pub notes: String,
}

impl ClanMembership {
    pub fn none() -> Self {
        ClanMembership {
            clan_id: 0,
            clan_name: String::new(),
            rank: 0,
            member_id: 0,
            notes: String::new(),
        }
    }

    pub fn is_leader(&self) -> bool {
        self.rank == ranks::CLAN_RANK_PRINCE || self.rank == ranks::CLAN_RANK_LEAGUE_PRINCE
    }

    pub fn is_guardian(&self) -> bool {
        self.rank == ranks::CLAN_RANK_GUARDIAN || self.rank == ranks::CLAN_RANK_LEAGUE_GUARDIAN
    }

    pub fn has_clan(&self) -> bool {
        self.clan_id > 0
    }

    /// Check if this rank can invite new members.
    pub fn can_invite(&self) -> bool {
        matches!(
            self.rank,
            ranks::CLAN_RANK_PRINCE
                | ranks::CLAN_RANK_GUARDIAN
                | ranks::CLAN_RANK_LEAGUE_PRINCE
                | ranks::CLAN_RANK_LEAGUE_GUARDIAN
                | ranks::CLAN_RANK_LEAGUE_VICEPRINCE
        )
    }
}

/// Adena cost to create a clan.
pub const CLAN_CREATE_COST: i32 = 30_000;

/// Adena item ID.
pub const ADENA_ITEM_ID: i32 = 40308;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_clan_data_members() {
        let mut clan = ClanData::new(1, "TestClan".into(), 100, "Leader".into());
        clan.add_member("Player1".into());
        clan.add_member("Player2".into());
        clan.add_member("Player1".into()); // duplicate

        assert_eq!(clan.member_count(), 2);

        clan.remove_member("Player1");
        assert_eq!(clan.member_count(), 1);
        assert_eq!(clan.member_names[0], "Player2");
    }

    #[test]
    fn test_clan_membership_ranks() {
        let leader = ClanMembership {
            clan_id: 1, clan_name: "Test".into(),
            rank: ranks::CLAN_RANK_PRINCE, member_id: 1, notes: String::new(),
        };
        assert!(leader.is_leader());
        assert!(leader.can_invite());
        assert!(!leader.is_guardian());

        let guardian = ClanMembership {
            clan_id: 1, clan_name: "Test".into(),
            rank: ranks::CLAN_RANK_GUARDIAN, member_id: 2, notes: String::new(),
        };
        assert!(!guardian.is_leader());
        assert!(guardian.is_guardian());
        assert!(guardian.can_invite());

        let public = ClanMembership {
            clan_id: 1, clan_name: "Test".into(),
            rank: ranks::CLAN_RANK_PUBLIC, member_id: 3, notes: String::new(),
        };
        assert!(!public.is_leader());
        assert!(!public.can_invite());
    }

    #[test]
    fn test_no_clan() {
        let none = ClanMembership::none();
        assert!(!none.has_clan());
        assert!(!none.is_leader());
    }
}
