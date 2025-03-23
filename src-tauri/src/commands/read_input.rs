use std::sync::Mutex;
use tauri::State;

use crate::AppData;

#[tauri::command]
pub fn read_input(input: &str, state: State<'_, Mutex<AppData>>) -> String {
    let mut app_data = state.lock().unwrap();

    format!("{}", app_data.game.actual_location.perform_action(input))
}
