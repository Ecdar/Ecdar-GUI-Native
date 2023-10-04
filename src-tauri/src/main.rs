// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
pub mod project;
use project::*;

fn main() {
  tauri::Builder::default()
    .setup(|app| {
        #[cfg(debug_assertions)]
        {
            app.get_window("main").unwrap().open_devtools();
        }
        Ok(())
    })
    .invoke_handler(tauri::generate_handler![
        load_project,
        save_project
    ])
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
