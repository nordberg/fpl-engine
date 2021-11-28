use crate::models::player::{Player, Position};

pub struct PlayerMatchReport {
    player: Player,

    position: Position,

    started_at_minute: u8,  // At what point player entered pitch. 0 for starting XI
    finished_at_minute: u8,  // At what point player left pitch.

    goals_scored: u8,
    goals_conceded: u8,
    assists: u8
}

impl PlayerMatchReport {
    fn minutes_played(&self) -> u8 {
        return self.finished_at_minute - self.started_at_minute;
    }
}
