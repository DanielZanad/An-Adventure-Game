use std::sync::Mutex;
use tauri::State;

use crate::AppData;

#[tauri::command]
pub fn move_location(location: &str, state: State<'_, Mutex<AppData>>) -> String {
    let mut app_data = state.lock().unwrap();

    app_data.game.add_player_previous_location(location);

    match app_data.game.move_to(location) {
        Ok(message) => message,
        Err(error_message) => error_message,
    }
}
