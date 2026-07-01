mod midi;

use midi::{MidiService};

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
        .manage(MidiService::new())
        .invoke_handler(tauri::generate_handler![
            midi::connect,
            midi::note_on,
            midi::note_off,
            midi::play_notes,
            midi::stop_notes
        ])
        .plugin(tauri_plugin_opener::init())
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}