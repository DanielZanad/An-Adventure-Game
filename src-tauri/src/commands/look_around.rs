use std::sync::Mutex;
use tauri::State;

use crate::AppData;

#[tauri::command]
pub fn look_around(state: State<'_, Mutex<AppData>>) -> Vec<String> {
    let mut app_data = state.lock().unwrap();

    app_data.game.get_actual_location().unwrap().look_around()
}
