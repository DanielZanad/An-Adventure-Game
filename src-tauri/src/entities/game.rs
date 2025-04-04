use std::collections::HashMap;

use super::{location::Location, player::Player};

pub struct Game {
    pub player: Player,
    pub actual_location: String,
    pub locations: HashMap<String, Location>,
}

impl Game {
    pub fn new(player: Player, locations: Vec<Location>, actual_location: String) -> Self {
        let location_map = locations
            .into_iter()
            .map(|location| (location.name.clone(), location))
            .collect();

        Game {
            player,
            actual_location,
            locations: location_map,
        }
    }
}

impl GameTrait for Game {
    fn get_actual_location(&mut self) -> Option<&mut Location> {
        if let Some(current_location) = self.locations.get_mut(&self.actual_location) {
            Some(current_location)
        } else {
            None
        }
    }

    fn move_to(&mut self, next_location: &str) -> Result<String, String> {
        if let Some(current_location) = self.locations.get(&self.actual_location) {
            if current_location
                .connections_names
                .contains(&next_location.to_string())
            {
                self.actual_location = next_location.to_string();
                Ok(format!(
                    "Você se moveu para {}",
                    self.actual_location.clone()
                ))
            } else {
                Err(format!(
                    "Não pode se mover de {} para {}",
                    self.actual_location, next_location
                ))
            }
        } else {
            Err("Localização atual não foi encontrada".to_string())
        }
    }

    fn add_player_previous_location(&mut self, location: &str) {
        self.player.previous_locations.push(location.to_string());
    }

    fn get_last_player_location(&mut self) -> Option<String> {
        if self.player.previous_locations.len() <= 1 {
            return None;
        }
        if let Some(&ref penultimate_location) = self
            .player
            .previous_locations
            .get(self.player.previous_locations.len().wrapping_sub(2))
        {
            return Some(penultimate_location.clone());
        }
        None
    }

    fn get_player_previous_locations(&self) -> Vec<String> {
        self.player.previous_locations.clone()
    }
}

pub trait GameTrait {
    fn get_actual_location(&mut self) -> Option<&mut Location>;
    fn move_to(&mut self, next_location: &str) -> Result<String, String>;
    fn add_player_previous_location(&mut self, location: &str);
    fn get_last_player_location(&mut self) -> Option<String>;
    fn get_player_previous_locations(&self) -> Vec<String>;
}
