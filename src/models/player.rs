#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Position {
    Gk,
    Def,
    Mid,
    Fwd,
}

#[derive(Clone, Eq, PartialEq, Hash)]
pub struct Player {
    name: String,
    position: Position,
    team_id: u32,
    id: u32,
}

impl Player {
    pub fn new(name: String, position: Position, team_id: u32, id: u32) -> Player {
        Player {
            name,
            position,
            team_id,
            id,
        }
    }

    pub fn team_id(&self) -> u32 {
        self.team_id
    }

    pub fn position(&self) -> Position {
        self.position
    }

    pub fn id(&self) -> u32 {
        self.id
    }
}
