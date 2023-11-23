// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;

#[derive(serde::Serialize, serde::Deserialize)]
pub enum GrpcError {
    FailedToConnect,
    FailedResponce,
}

ecdar_gui_macros::create_functions!();

fn main() {
    tauri::Builder::default()
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                app.get_window("main").unwrap().open_devtools();
            }
            Ok(())
        })
        .invoke_handler(ecdar_gui_macros::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
