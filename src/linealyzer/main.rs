#![feature(plugin)]
#![plugin(phf_macros)]

extern crate phf;

extern crate soundalyzer;
mod syscallmap;

use soundalyzer::midi_backend::MIDIBackend;
use soundalyzer::Backend;

use std::error::Error;
use std::io::{stdin, BufRead};

use syscallmap::to_note;

type line_to_note = fn(&str) -> u64;

fn run() -> Result<(), Box<Error>> {
    let mut backend = MIDIBackend::new()?;
    let input = stdin();
    for line in input.lock().lines() {
        let note = to_note(&line?);
        backend.play_note(note.unwrap_or(0) as u64)
    }
    Ok(())
}

fn main() {
    match run() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description()),
    }
}
