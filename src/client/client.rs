use std::collections::HashSet;

use crate::models::football_match::FootballMatch;
use crate::models::player::Player;
use crate::models::team::PlayingTeam;

pub trait FootballStatsClient {
    fn get_matches(&self, game_week: Option<u8>) -> Vec<&FootballMatch>;

    fn get_players(&self) -> &HashSet<Player>;

    fn get_teams(&self) -> &HashSet<PlayingTeam>;
}
