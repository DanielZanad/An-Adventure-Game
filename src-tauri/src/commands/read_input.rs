use std::sync::Mutex;
use tauri::State;

use crate::AppData;

#[tauri::command]
pub fn read_input(input: &str, state: State<'_, Mutex<AppData>>) -> String {
    let app_data = state.lock().unwrap();

    match &app_data.game {
        Some(game) => {
            format!(
                "{}",
                game.lock()
                    .unwrap()
                    .get_actual_location()
                    .unwrap()
                    .perform_action(input)
            )
        }
        None => String::from(""),
    }
}
