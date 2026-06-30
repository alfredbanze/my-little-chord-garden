mod midi;

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    midi::list_outputs();

    tauri::Builder::default()
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}