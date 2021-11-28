use std::collections::HashSet;

use rand::seq::SliceRandom;
use rand::RngCore;

use crate::client::client::FootballStatsClient;
use crate::models::football_match::FootballMatch;
use crate::models::player::{Player, Position};
use crate::models::team::PlayingTeam;

pub struct LocalFootballStatsClient<'a> {
    teams: HashSet<PlayingTeam>,
    matches: Vec<&'a FootballMatch<'a>>,
}

impl LocalFootballStatsClient<'_> {
    pub fn new() -> LocalFootballStatsClient<'static> {
        let teams = (0..20)
            .map(|i| create_team_with_players(None, None, i))
            .collect();
        LocalFootballStatsClient {
            teams,
            matches: Vec::new(),
        }
    }
}

impl FootballStatsClient for LocalFootballStatsClient<'_> {
    fn get_matches(&self, game_week: Option<u8>) -> Vec<&FootballMatch> {
        return match game_week {
            None => self.matches.iter().map(|m| *m).collect(),
            Some(game_week) => self
                .matches
                .iter()
                .filter(|m| m.get_game_week() == game_week)
                .map(|m| *m)
                .collect(),
        };
    }

    fn get_players(&self) -> &HashSet<Player> {
        todo!();
    }

    fn get_teams(&self) -> &HashSet<PlayingTeam> {
        return &self.teams;
    }
}

fn create_match(
    home_team: PlayingTeam,
    away_team: PlayingTeam,
    game_week: u8,
) -> FootballMatch<'static> {
    FootballMatch::new(home_team, away_team, vec![], game_week)
}

pub fn create_team_with_players(
    name: Option<String>,
    players: Option<HashSet<Player>>,
    id: u32,
) -> PlayingTeam {
    let team_name = name.unwrap_or(format!("Team {}", id));

    let first_names = vec!["Marcus", "Daniel", "William", "José", "Fred", "Ricardo"];
    let sur_names = vec![
        "Santos",
        "Rodriguez",
        "Lopez",
        "Martinez",
        "Garcia",
        "Hernandez",
    ];

    let mut generated_players: HashSet<Player> = HashSet::new();

    let mut rng = rand::thread_rng();

    let goalkeeper = Player::new(
        format!("Goalkeeper Keeperson"),
        Position::GK,
        id,
        rng.next_u32(),
    );
    generated_players.insert(goalkeeper);

    (0..4).for_each(|_| {
        let first_name = first_names.choose(&mut rng).unwrap();
        let sur_name = sur_names.choose(&mut rng).unwrap();
        let full_name = format!("{} {}", first_name, sur_name);
        let player = Player::new(full_name, Position::DEF, id, rng.next_u32());
        generated_players.insert(player);
    });

    (0..4).for_each(|_| {
        let first_name = first_names.choose(&mut rng).unwrap();
        let sur_name = sur_names.choose(&mut rng).unwrap();
        let full_name = format!("{} {}", first_name, sur_name);
        let player = Player::new(full_name, Position::MID, id, rng.next_u32());
        generated_players.insert(player);
    });

    (0..2).for_each(|_| {
        let first_name = first_names.choose(&mut rng).unwrap();
        let sur_name = sur_names.choose(&mut rng).unwrap();
        let full_name = format!("{} {}", first_name, sur_name);
        let player = Player::new(full_name, Position::FWD, id, rng.next_u32());
        generated_players.insert(player);
    });

    return PlayingTeam::new(team_name, id, players.unwrap_or(generated_players));
}
