// Prevents additional console window on Windows in release, DO NOT REMOVE!!
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use tauri::Manager;
use tauri_plugin_positioner::{Position, WindowExt};

fn main() {
    tauri::Builder::default()
        .plugin(tauri_plugin_positioner::init())
        .setup(|app| {
            // Get the main window
            let main_window = app.get_webview_window("main").unwrap();

            // Position the window (choose one of these)
            // let _ = main_window.move_window(Position::TopRight);
            // let _ = main_window.move_window(Position::Center);
            // let _ = main_window.move_window(Position::BottomLeft);
            // let _ = main_window.move_window(Position::TopLeft);
            let _ = main_window.move_window(Position::BottomRight);

            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
