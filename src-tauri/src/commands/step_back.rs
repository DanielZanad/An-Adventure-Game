use std::sync::Mutex;
use tauri::State;

use crate::AppData;

#[tauri::command]
pub fn step_back(state: State<'_, Mutex<AppData>>) -> String {
    let mut app_data = state.lock().unwrap();

    match &app_data.game {
        Some(game) => {
            let mut game_borrowed = game.lock().unwrap();

            let last_player_location = game_borrowed.get_last_player_location();

            if let Some(message) = last_player_location {
                game_borrowed.add_player_previous_location(&message);
            }

            let last_player_location = game_borrowed.get_player_previous_locations().clone()
                [game_borrowed.get_player_previous_locations().len() - 1]
                .clone();

            match game_borrowed.move_to(&last_player_location) {
                Ok(_) => format!("Você voltou para {}", last_player_location),
                Err(error_message) => error_message,
            }
        }
        None => String::from(""),
    }
}
