use std::sync::{Arc, Mutex};

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use commands::look_around::look_around;
use commands::read_input::read_input;
use commands::start_game::start_game;
use entities::{
    action::Actions,
    character::{Character, Dialog},
    game::{Game, GameTrait},
    location::Location,
    player::Player,
};
use tauri::Manager;

pub mod commands;
pub mod entities;
pub mod helpers;

pub struct AppData {
    game: Option<Arc<Mutex<dyn GameTrait + Send + Sync>>>,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .setup(|app| {
            app.manage(AppData { game: None });
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![
            read_input,
            look_around,
            start_game
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
