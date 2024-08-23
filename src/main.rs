use midir::{MidiOutput, MidiOutputConnection};
use std::error::Error;
use std::thread::sleep;
use std::time::Duration;

fn main() -> Result<(), Box<dyn Error>> {
    // Initialize the MIDI output
    println!("Initializing MIDI output...");
    let midi_out = MidiOutput::new("Launchpad X")?;
    
    // List available MIDI output ports
    let out_ports = midi_out.ports();
    println!("Available output ports: {}", out_ports.len());

    if out_ports.is_empty() {
        return Err("No available MIDI output ports".into());
    }

    //list available ports and names
    for (i, port) in out_ports.iter().enumerate() {
        let port_name = midi_out.port_name(port)?;
        println!("Port {}: {}", i, port_name);
    }

    // Select the Launchpad MIDI port
    let port_index = 2; // Assuming LPX MIDI is Port 1, you can adjust as needed
    let port = &out_ports[port_index];
    let port_name = midi_out.port_name(port)?;
    println!("Selected output port: {}", port_name);

    // Connect to the selected port
    println!("Connecting to the selected port...");
    let mut conn_out = midi_out.connect(port, "Launchpad out")?;
    println!("Connected successfully!");

    // Step 1: Switch to Programmer Mode (SysEx)
    let programmer_mode_sysex = [
        0xF0, 0x00, 0x20, 0x29, 0x02, 0x0C, 0x0E, 0x01, 0xF7
    ];
    println!("Sending SysEx message to enter Programmer Mode...");
    conn_out.send(&programmer_mode_sysex)?;
    println!("Programmer Mode enabled.");

    // Step 2: Light up a pad (lower left pad to static red)
    let note_on_msg = [0xF0, 0x00, 0x20, 0x29, 0x02, 0x0C, 0x03, 0x00, 0x63, 0x15, 0xF7]; // Note On, Pad 0B (11), Velocity 5 (Red)
    println!("Sending sysex On message to light up the lower left pad (red)...");
    conn_out.send(&note_on_msg)?;
    println!("Note On message sent.");

    // let note_on_msg = [0x90, 0x0B, 0x15]; 
    // println!("Sending Note On message to light up the lower left pad (red)...");
    // conn_out.send(&note_on_msg)?;
    // println!("Note On message sent.");

    // Wait for 1 second
    sleep(Duration::from_secs(10));

    // Step 3: Turn off the pad
    let note_off_msg = [0x80, 0x63, 0x00]; // Note Off, Pad 0B (11), Velocity 0 (off)
    println!("Sending Note Off message to turn off the pad...");
    conn_out.send(&note_off_msg)?;
    println!("Note Off message sent.");

    println!("Done!");

    Ok(())
}
