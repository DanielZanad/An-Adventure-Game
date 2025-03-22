// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use adventure_game_lib;

fn main() {
    adventure_game_lib::run()
}
