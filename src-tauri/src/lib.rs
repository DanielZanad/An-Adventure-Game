use std::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use commands::look_around::look_around;
use commands::read_input::read_input;
use entities::{
    action::Actions,
    character::{Character, Dialog},
    game::Game,
    location::Location,
    player::Player,
};
use tauri::Manager;

pub mod commands;
pub mod entities;
pub mod helpers;

pub struct AppData {
    game: Game,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppData { game }));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![read_input, look_around])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
