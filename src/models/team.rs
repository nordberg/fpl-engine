use std::collections::HashSet;

use crate::models::player::Player;

#[derive(Eq, PartialEq, Clone)]
pub struct PlayingTeam {
    name: String,
    players: HashSet<Player>,
    id: u32,
}

impl PlayingTeam {
    pub fn new(name: String, id: u32, players: HashSet<Player>) -> Self {
        PlayingTeam { name, players, id }
    }

    pub fn get_id(&self) -> u32 {
        self.id
    }
}
