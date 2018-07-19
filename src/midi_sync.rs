extern crate midir;

use super::Backend;
use std::error::Error;
use std::io::{stdin, stdout, Write};
use std::thread::sleep;
use std::time::Duration;

use midir::MidiOutput;

const NOTE_ON_MSG: u8 = 0x90;
const NOTE_OFF_MSG: u8 = 0x80;
const VELOCITY: u8 = 127;
const NOTE_DURATION: u64 = 100;

pub struct MIDISync {
    conn: midir::MidiOutputConnection,
}

impl MIDISync {
    pub fn new() -> Result<Self, Box<Error>> {
        let midi_out = MidiOutput::new("My Test Output")?;

        let out_port = match midi_out.port_count() {
            0 => return Err("no output port found".into()),
            _ => {
                println!(
                    "Choosing the only available output port: {}",
                    midi_out.port_name(0).unwrap()
                );
                0
            }
        };

        println!("\nOpening connection");
        let mut conn_out = midi_out.connect(out_port, "midir-test")?;
        println!("Connection open. Listen!");

        Ok(MIDISync { conn: conn_out })
    }
}

impl Backend for MIDISync {
    fn play_note(&mut self, note: u64) {
        let note = 20 + (note % 107) as u8;
        let _ = self.conn.send(&[NOTE_ON_MSG, note, VELOCITY]);
        sleep(Duration::from_millis(NOTE_DURATION));
        let _ = self.conn.send(&[NOTE_OFF_MSG, note, VELOCITY]);
    }
}
