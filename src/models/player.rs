#[derive(Clone, Copy, Eq, PartialEq, Hash)]
pub enum Position {
    GK,
    DEF,
    MID,
    FWD,
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
        return self.team_id;
    }

    pub fn position(&self) -> Position {
        return self.position;
    }

    pub fn id(&self) -> u32 {
        return self.id;
    }
}
