#![feature(plugin)]
#![plugin(phf_macros)]

extern crate clap;
extern crate phf;
extern crate soundalyzer;

mod syscallmap;

use clap::{App, Arg};

use soundalyzer::midi_backend::MIDIBackend;
use soundalyzer::Backend;

use std::error::Error;
use std::io::{stdin, BufRead};

type line_to_note = fn(&str) -> u64;

fn syscall() -> Result<(), Box<Error>> {
    let mut backend = MIDIBackend::new()?;
    let input = stdin();
    for line in input.lock().lines() {
        let note = syscallmap::to_note(&line?);
        backend.play_note(note.unwrap_or(0) as u64)
    }
    Ok(())
}
fn main() {
    let matches = App::new("Taskalyzer")
        .version("0.0")
        .author("Denis Lavrov <bahus.vel@gmail.com>")
        .about("Like strace but outputs sounds")
        .arg(
            Arg::with_name("input")
                .default_value("-")
                .required(true)
                .help("File like source to take the lines from"),
        )
        .get_matches();

    match syscall() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description()),
    }
}
