use midir::MidiOutput;

pub fn list_outputs() {
    let midi_out = MidiOutput::new("My Little Chord Garden").unwrap();

    let ports = midi_out.ports();

    if ports.is_empty() {
        println!("Kein MIDI-Ausgang gefunden.");
        return;
    }

    let port = &ports[0];

    let name = midi_out.port_name(port).unwrap();

    println!("Verbunden mit: {}", name);

    let _conn = midi_out
        .connect(port, "My Little Chord Garden")
        .unwrap();

    println!("MIDI-Verbindung geöffnet.");
}