extern crate midir;

use std::borrow::Borrow;
use std::cmp::{max, min};
use std::error::Error;
use std::io::{stdin, BufRead};
use std::sync::{Arc, Mutex};
use std::thread;
use std::thread::sleep;
use std::time::Duration;

use super::Backend;
use midir::MidiOutput;

const NOTE_ON_MSG: u8 = 0x90;
const NOTE_OFF_MSG: u8 = 0x80;
const VELOCITY_STEP: u8 = 40;
const NOTE_DURATION: u64 = 50;

struct BackendState {
    notes: [u8; 127],
    conn: Option<midir::MidiOutputConnection>,
    running: bool,
}

pub struct MIDIBackend {
    state: Arc<Mutex<BackendState>>,
}

impl Backend for MIDIBackend {
    fn play_note(&mut self, note: u64) {
        let mut lock = self.state.lock().unwrap();
        let note = (20 + (note % 107)) as u8;
        let nvel = min(lock.notes[note as usize] + VELOCITY_STEP, 127);
        lock.notes[note as usize] = nvel;
        let _ = lock.conn.as_mut().unwrap().send(&[NOTE_ON_MSG, note, nvel]);
    }
}

impl Drop for MIDIBackend {
    fn drop(&mut self) {
        let mut lock = self.state.lock().unwrap();
        lock.running = false;
        for i in 0..127 {
            let _ = lock
                .conn
                .as_mut()
                .unwrap()
                .send(&[NOTE_OFF_MSG, i as u8, 127]);
        }
    }
}

impl MIDIBackend {
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

        let state = Arc::new(Mutex::new(BackendState {
            notes: [0; 127],
            conn: Some(conn_out),
            running: true,
        }));

        let backend = MIDIBackend {
            state: state.clone(),
        };
        thread::spawn(move || epoch_thread(state));

        return Ok(backend);
    }
}

fn epoch_thread(state: Arc<Mutex<BackendState>>) {
    loop {
        sleep(Duration::from_millis(NOTE_DURATION));
        let mut lock = state.lock().unwrap();
        if lock.running == false {
            return;
        }
        for i in 0..127 {
            if lock.notes[i] > 0 {
                lock.notes[i] = 0;
                if lock.notes[i] != 0 {
                    continue;
                }
                let _ = lock
                    .conn
                    .as_mut()
                    .unwrap()
                    .send(&[NOTE_OFF_MSG, i as u8, 127]);
            }
        }
        drop(lock);
    }
}
