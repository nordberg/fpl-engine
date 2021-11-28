use crate::client::client::FootballStatsClient;
use crate::models::football_match::FootballMatch;
use crate::models::match_event::{MatchEvent, MatchEventType};
use crate::models::player::{Player, Position};

pub struct PointCalculatorService {
    pub(crate) client: Box<dyn FootballStatsClient>,
}

impl PointCalculatorService {
    pub fn new(client: Box<dyn FootballStatsClient>) -> Box<PointCalculatorService> {
        return Box::new(PointCalculatorService { client });
    }

    pub fn points_for_player_in_gw(&self, player_id: u32, game_week: u8) -> Option<i8> {
        let player = self
            .client
            .get_players()
            .iter()
            .find(|p| p.id() == player_id)
            .unwrap();

        let relevant_match_events = self
            .client
            .get_matches(Some(game_week))
            .iter()
            .filter(|m| {
                m.get_home_team().get_id() == player.team_id()
                    || m.get_away_team().get_id() == player.team_id()
            })
            .flat_map(|m| {
                m.get_match_events()
                    .iter()
                    .filter(|me| me.get_player().id() == player.id())
            })
            .collect();

        return PointCalculatorService::points_for_player_in_match_events(
            relevant_match_events,
            player,
        );
    }

    fn points_for_player_in_match_events(
        match_events: Vec<&MatchEvent>,
        player: &Player,
    ) -> Option<i8> {
        return match_events.iter().fold(Option::None, |acc, me| {
            if me.get_player().id() != player.id() {
                return acc;
            }
            let points = PointCalculatorService::points_for_position(me, player.position());
            return match acc {
                None => Option::Some(points),
                Some(acc_points) => Option::Some(acc_points + points),
            };
        });
    }

    fn points_for_position(match_event: &MatchEvent, position: Position) -> i8 {
        return match position {
            Position::GK => PointCalculatorService::points_for_goalkeeper(match_event.get_type()),
            Position::DEF => PointCalculatorService::points_for_defender(match_event.get_type()),
            Position::MID => PointCalculatorService::points_for_midfielder(match_event.get_type()),
            Position::FWD => PointCalculatorService::points_for_forward(match_event.get_type()),
        };
    }

    fn points_for_goalkeeper(match_event_type: MatchEventType) -> i8 {
        return match match_event_type {
            MatchEventType::Goal => 6,
            MatchEventType::Assist => 3,
            MatchEventType::Save => 1,
            MatchEventType::RedCard => -3,
            MatchEventType::YellowCard => -1,
        };
    }

    fn points_for_defender(match_event_type: MatchEventType) -> i8 {
        return match match_event_type {
            MatchEventType::Goal => 6,
            MatchEventType::Assist => 3,
            MatchEventType::Save => 0,
            MatchEventType::RedCard => -3,
            MatchEventType::YellowCard => -1,
        };
    }

    fn points_for_midfielder(match_event_type: MatchEventType) -> i8 {
        return match match_event_type {
            MatchEventType::Goal => 5,
            MatchEventType::Assist => 3,
            MatchEventType::Save => 0,
            MatchEventType::RedCard => -3,
            MatchEventType::YellowCard => -1,
        };
    }

    fn points_for_forward(match_event_type: MatchEventType) -> i8 {
        return match match_event_type {
            MatchEventType::Goal => 4,
            MatchEventType::Assist => 3,
            MatchEventType::Save => 0,
            MatchEventType::RedCard => -3,
            MatchEventType::YellowCard => -1,
        };
    }
}

#[cfg(test)]
mod tests {
    use crate::models::football_match::FootballMatch;
    use crate::models::match_event::{MatchEvent, MatchEventType};
    use crate::models::player::{Player, Position};
    use crate::PointCalculatorService;

    #[test]
    fn two_players_only_get_points_for_their_own_actions() {
        let player1 = Player::new(String::from("Marcus"), Position::FWD, 1, 1);
        let player2 = Player::new(String::from("William"), Position::MID, 1, 2);
        let match_events = vec![
            MatchEvent::new(MatchEventType::Goal, 15, &player1),
            MatchEvent::new(MatchEventType::Assist, 15, &player2),
        ];

        let player1_points = PointCalculatorService::points_for_player_in_match_events(
            match_events.iter().collect(),
            &player1,
        );

        let player2_points = PointCalculatorService::points_for_player_in_match_events(
            match_events.iter().collect(),
            &player2,
        );

        assert_eq!(player1_points, Some(4));
        assert_eq!(player2_points, Some(3));
    }

    #[test]
    fn player_without_match_events_get_none() {
        let events: Vec<MatchEvent> = vec![];
        let player = Player::new(String::from("Marcus"), Position::FWD, 1, 1);

        let points = PointCalculatorService::points_for_player_in_match_events(
            events.iter().collect(),
            &player,
        );

        assert_eq!(points, None);
    }

    #[test]
    fn player_with_negative_actions_get_minus_points() {
        let player = Player::new(String::from("Marcus"), Position::FWD, 1, 1);
        let events = vec![MatchEvent::new(MatchEventType::RedCard, 15, &player)];

        let points = PointCalculatorService::points_for_player_in_match_events(
            events.iter().collect(),
            &player,
        );

        assert_eq!(points, Some(-3));
    }
}
