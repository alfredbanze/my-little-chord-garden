use midir::{MidiOutput, MidiOutputConnection};
use std::sync::Mutex;
use tauri::State;

const APP_NAME: &str = "My Little Chord Garden";

pub struct MidiService {
    connection: Mutex<Option<MidiOutputConnection>>,
}

impl MidiService {
    pub fn new() -> Self {
        Self {
            connection: Mutex::new(None),
        }
    }
}

fn note_message(status: u8, note: u8) -> [u8; 3] {
    [status, note, 0x40]
}

#[tauri::command]
pub fn connect(service: State<'_, MidiService>) -> Result<String, String> {
    let midi_out = MidiOutput::new(APP_NAME)
        .map_err(|err| format!("Failed to initialize MIDI output: {err}"))?;

    let ports = midi_out.ports();
    if ports.is_empty() {
        return Err("No MIDI output ports found".into());
    }

    let port = ports.first().ok_or("No MIDI output ports found")?;
    let port_name = midi_out
        .port_name(port)
        .map_err(|err| format!("Failed to read MIDI port name: {err}"))?;

    let connection = midi_out
        .connect(port, APP_NAME)
        .map_err(|err| format!("Failed to connect to MIDI output: {err}"))?;

    let mut guard = service
        .connection
        .lock()
        .map_err(|err| format!("Failed to lock MIDI state: {err}"))?;
    *guard = Some(connection);

    Ok(format!("Connected to {port_name}"))
}

#[tauri::command]
pub fn note_on(note: u8, service: State<'_, MidiService>) -> Result<(), String> {
    send_note(0x90, note, service)
}

#[tauri::command]
pub fn note_off(note: u8, service: State<'_, MidiService>) -> Result<(), String> {
    send_note(0x80, note, service)
}

#[tauri::command]
pub fn play_notes(notes: Vec<u8>, service: State<'_, MidiService>) -> Result<(), String> {
    for note in notes {
        send_note(0x90, note, service.clone())?;
    }

    Ok(())
}

#[tauri::command]
pub fn stop_notes(notes: Vec<u8>, service: State<'_, MidiService>) -> Result<(), String> {
    for note in notes {
        send_note(0x80, note, service.clone())?;
    }

    Ok(())
}

fn send_note(status: u8, note: u8, service: State<'_, MidiService>) -> Result<(), String> {
    let mut guard = service
        .connection
        .lock()
        .map_err(|err| format!("Failed to lock MIDI state: {err}"))?;
    let connection = guard
        .as_mut()
        .ok_or_else(|| "No MIDI output connected".to_string())?;

    connection
        .send(&note_message(status, note))
        .map_err(|err| format!("Failed to send MIDI message: {err}"))?;

    Ok(())
}

#[cfg(test)]
mod tests {
    use super::note_message;

    #[test]
    fn note_message_uses_status_velocity_and_pitch() {
        assert_eq!(note_message(0x90, 60), [0x90, 60, 0x40]);
        assert_eq!(note_message(0x80, 60), [0x80, 60, 0x40]);
    }
}