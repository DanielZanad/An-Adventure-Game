use std::sync::Mutex;
use tauri::State;

use crate::AppData;

#[tauri::command]
pub fn look_around(state: State<'_, Mutex<AppData>>) -> Vec<String> {
    let app_data = state.lock().unwrap();

    match &app_data.game {
        Some(game) => game
            .lock()
            .unwrap()
            .get_actual_location()
            .unwrap()
            .look_around(),
        None => vec![String::from("")],
    }
}
