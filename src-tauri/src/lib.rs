mod midi;

use midi::{connect, note_off, note_on, MidiService};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(MidiService::new())
        .invoke_handler(tauri::generate_handler![connect, note_on, note_off])
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}