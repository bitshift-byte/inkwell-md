use std::collections::HashMap;
use std::sync::Mutex;
use notify::RecommendedWatcher;

mod commands;

pub struct WatcherState {
    pub watchers: Mutex<HashMap<String, RecommendedWatcher>>,
}

pub fn run() {
    tauri::Builder::default()
        .plugin(tauri_plugin_dialog::init())
        .plugin(tauri_plugin_fs::init())
        .plugin(tauri_plugin_shell::init())
        .manage(WatcherState {
            watchers: Mutex::new(HashMap::new()),
        })
        .setup(|_app| {
            #[cfg(debug_assertions)]
            {
                use tauri::Manager;
                let window = _app.get_webview_window("main").unwrap();
                window.open_devtools();
            }
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            commands::read_directory,
            commands::read_file,
            commands::save_file,
            commands::get_app_info,
            commands::watch_directory,
            commands::unwatch_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running Inkwell MD");
}
