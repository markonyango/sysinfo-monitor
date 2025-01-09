#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

mod state;

use monoxide_backend::{docker::DockerStats, MonitorData};
use state::{AppState, Results};
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
        .invoke_handler(tauri::generate_handler![
            update_all,
            update_disks,
            update_network,
            update_system,
            update_processes,
            update_docker_stats
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}

#[tauri::command]
async fn update_all(state: State<'_, Mutex<AppState>>) -> Result<Results, String> {
    state.lock().await.update_all().await
}

#[tauri::command]
async fn update_disks(state: State<'_, Mutex<AppState>>) -> Result<MonitorData, String> {
    Ok(state.lock().await.update_disks())
}
#[tauri::command]
async fn update_network(state: State<'_, Mutex<AppState>>) -> Result<MonitorData, String> {
    Ok(state.lock().await.update_network())
}
#[tauri::command]
async fn update_system(state: State<'_, Mutex<AppState>>) -> Result<MonitorData, String> {
    Ok(state.lock().await.update_system())
}
#[tauri::command]
async fn update_processes(state: State<'_, Mutex<AppState>>) -> Result<MonitorData, String> {
    Ok(state.lock().await.update_processes())
}

#[tauri::command]
async fn update_docker_stats(state: State<'_, Mutex<AppState>>) -> Result<MonitorData, String> {
    Ok(state.lock().await.update_docker_stats().await)
}
