use std::collections::HashSet;
use std::hash::{Hash, Hasher};

use crate::models::player::{Player};

#[derive(Eq, PartialEq, Clone)]
pub struct PlayingTeam {
    name: String,
    players: HashSet<Player>,
    id: u32
}

impl PlayingTeam {
    pub fn new(name: String, id: u32, players: HashSet<Player>) -> Self {
        PlayingTeam {
            name,
            players,
            id
        }
    }

    pub fn get_id(&self) -> u32 {
        return self.id;
    }
}

impl Hash for PlayingTeam {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

