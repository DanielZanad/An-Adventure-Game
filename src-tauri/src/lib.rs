use std::sync::Mutex;

// Learn more about Tauri commands at https://tauri.app/develop/calling-rust/
use entities::{actions::Actions, location::Location};
use tauri::{Manager, State};

pub mod entities;
pub mod helpers;

#[tauri::command]
fn greet(input: &str, state: State<'_, Mutex<AppData>>) -> String {
    let app_data = state.lock().unwrap();
    app_data.location.show_actions();

    format!(
        "Hello, {}! You've been greeted from Rust!",
        app_data.location.perform_action(input)
    )
}

struct AppData {
    location: Location,
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    let action1 = Actions::new(
        "checar bolso",
        vec!["olharbolso", "checarbolso"],
        "Olhar bolso",
        "Voce olhou seu bolso e pegou uma chave",
        Some("fail_message".to_string()),
    );
    let action2 = Actions::new(
        "usar chave",
        vec!["usarchave", "pegarchave"],
        "Usando a chave",
        "Voce destrancou a porta com a chave",
        Some("Voce precisa pegar a chave, tente usar 'checar bolso' antes".to_string()),
    );
    let action3 = Actions::new(
        "Abrir porta",
        vec!["abrirporta", "virarmacaneta"],
        "Abrindo a porta",
        "Voce abriu a porta",
        Some("Voce precisa de uma chave para abrir a porta".to_string()),
    );

    let mut location = Location::new("Quarto");

    location.add_action(action1);
    location.add_action(action2);
    location.add_action(action3);
    tauri::Builder::default()
        .setup(|app| {
            app.manage(Mutex::new(AppData { location }));
            Ok(())
        })
        .plugin(tauri_plugin_opener::init())
        .invoke_handler(tauri::generate_handler![greet])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
