use std::sync::Mutex;
use tauri::State;

use crate::AppData;

#[tauri::command]
pub fn step_back(state: State<'_, Mutex<AppData>>) -> String {
    let mut app_data = state.lock().unwrap();
    let last_player_location = app_data.game.get_last_player_location();

    if let Some(message) = last_player_location {
        app_data.game.add_player_previous_location(&message);
    }

    let last_player_location = app_data.game.player.previous_locations
        [app_data.game.player.previous_locations.len() - 1]
        .clone();

    match app_data.game.move_to(&last_player_location) {
        Ok(_) => format!("Você voltou para {}", last_player_location),
        Err(error_message) => error_message,
    }
}
