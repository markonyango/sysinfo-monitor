#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;

use state::AppState;
use tauri::{async_runtime::Mutex, State};

/// Main entry point for the application
///
/// # Panics
///
/// Will panic if:
/// - Unable to create the main window
/// - Failed to apply window effects
/// - Failed to initialize the application state
fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_shell::init())
        .plugin(tauri_plugin_os::init())
        .manage(Mutex::new(AppState::new()))
        .invoke_handler(tauri::generate_handler![update_all,])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn update_all(state: State<'_, Mutex<AppState>>) -> Result<serde_json::Value, String> {
    Ok(state.lock().await.update_all().await)
}
