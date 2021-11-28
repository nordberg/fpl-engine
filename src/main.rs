use crate::client::local_client::LocalFootballStatsClient;
use crate::service::point_calculator_service::PointCalculatorService;

mod client;
mod models;
mod service;

fn main() {
    let local_client = Box::new(LocalFootballStatsClient::new());
    let point_calculator_service = PointCalculatorService::new(local_client);
    point_calculator_service.points_for_player_in_gw(1, 1);
}
