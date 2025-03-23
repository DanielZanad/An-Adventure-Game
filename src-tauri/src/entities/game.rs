use super::{location::Location, player::Player};

pub struct Game {
    player: Player,
    pub actual_location: Location,
    pub locations: Vec<Location>,
}

impl Game {
    pub fn new(player: Player, location: Vec<Location>, actual_location: Location) -> Self {
        Game {
            player,
            actual_location,
            locations: location,
        }
    }
}
