use crate::models::player::Player;

#[derive(Eq, PartialEq, Clone, Copy)]
pub enum MatchEventType {
    Goal,
    Assist,
    Save,
    RedCard,
    YellowCard,
}

#[derive(Eq)]
pub struct MatchEvent<'a> {
    event_type: MatchEventType,
    minute: u8,
    player: &'a Player,
}

impl MatchEvent<'_> {
    pub fn new(event_type: MatchEventType, minute: u8, player: &Player) -> MatchEvent {
        MatchEvent {
            event_type,
            minute,
            player,
        }
    }

    pub fn get_player(&self) -> &Player {
        &self.player
    }

    pub fn get_type(&self) -> MatchEventType {
        self.event_type
    }
}

impl PartialEq for MatchEvent<'_> {
    fn eq(&self, other: &Self) -> bool {
        self.minute == other.minute
    }
}
