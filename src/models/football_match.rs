use crate::models::match_event::MatchEvent;
use crate::models::team::PlayingTeam;

pub struct FootballMatch<'a> {
    home_team: PlayingTeam,
    away_team: PlayingTeam,
    match_events: Vec<MatchEvent<'a>>,
    game_week: u8,
}

impl FootballMatch<'_> {
    pub fn new(
        home_team: PlayingTeam,
        away_team: PlayingTeam,
        match_events: Vec<MatchEvent>,
        game_week: u8,
    ) -> FootballMatch {
        FootballMatch {
            home_team,
            away_team,
            match_events,
            game_week,
        }
    }

    pub fn get_game_week(&self) -> u8 {
        return self.game_week;
    }

    pub fn get_home_team(&self) -> &PlayingTeam {
        return &self.home_team;
    }

    pub fn get_away_team(&self) -> &PlayingTeam {
        return &self.away_team;
    }

    pub fn get_match_events(&self) -> &[MatchEvent<'_>] {
        return self.match_events.as_slice();
    }
}
