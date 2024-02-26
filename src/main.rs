// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
tonic::include_proto!("ecdar_proto_buf");

#[derive(serde::Serialize, serde::Deserialize)]
pub enum GrpcError {
    FailedToConnect,
    FailedResponse,
}

//ecdar_app::create_functions!();

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_persisted_scope::init())
        .setup(|app| {
            #[cfg(debug_assertions)]
            {
                app.get_window("main").unwrap().open_devtools();
            }
            Ok(())
        })
        //.invoke_handler(ecdar_app::generate_handler![])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
