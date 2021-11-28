use std::collections::HashSet;

use crate::client::client::FootballStatsClient;
use crate::models::football_match::FootballMatch;
use crate::models::player::Player;
use crate::models::team::PlayingTeam;

struct PremierLeagueFootballStatsClient {}

impl FootballStatsClient for PremierLeagueFootballStatsClient {
    fn get_matches(&self, game_week: Option<u8>) -> Vec<&FootballMatch> {
        todo!()
    }

    fn get_players(&self) -> &HashSet<Player> {
        todo!()
    }

    fn get_teams(&self) -> &HashSet<PlayingTeam> {
        todo!()
    }
}
