#![feature(plugin)]
#![plugin(phf_macros)]

extern crate clap;
extern crate phf;
extern crate soundalyzer;

mod metaphone;
mod syscallmap;

use clap::{App, Arg};

use soundalyzer::midi_backend::MIDIBackend;
use soundalyzer::midi_sync::MIDISync;
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

fn phonetic() -> Result<(), Box<Error>> {
    let mut backend = MIDISync::new()?;
    let input = stdin();
    for line in input.lock().lines() {
        for c in line?
            .to_lowercase()
            .chars()
            .filter(|c| c.is_ascii() && c.is_alphabetic())
        {
            // shift to 0
            backend.play_note((c as u8 - 97) as u64)
        }
    }
    Ok(())
}

fn metaphone() -> Result<(), Box<Error>> {
    let mut backend = MIDISync::new()?;
    let input = stdin();
    for line in input.lock().lines() {
        let mut line = line.unwrap();
        line.retain(|c| c.is_ascii() && (c.is_alphabetic() || c.is_whitespace()));
        for word in line.split_whitespace() {
            let meta = metaphone::metaphone(word);
            print!("{:?} ", meta);
            for note in meta.chars().map(metaphone::to_note).map(|o| o.unwrap_or(0)) {
                backend.play_note(note)
            }
        }
        println!("");
    }
    Ok(())
}

fn main() {
    let matches = App::new("Soundalizer")
        .version("0.0")
        .author("Denis Lavrov <bahus.vel@gmail.com>")
        .about("Transforms line buffered output to sounds")
        .arg(
            Arg::with_name("mode")
                .short("m")
                .long("mode")
                .possible_values(&["syscall", "phonetic", "metaphone"])
                .default_value("metaphone")
                .help("Switches soundalizer to use a different algorithm")
                .takes_value(true),
        )
        .arg(
            Arg::with_name("input")
                .default_value("-")
                .required(true)
                .help("File like source to take the lines from"),
        )
        .arg(
            Arg::with_name("output")
                .default_value("-")
                .required(false)
                .help("File like destination to output compressed data"),
        )
        .get_matches();

    let res = match matches.value_of("mode") {
        Some("syscall") => syscall(),
        Some("phonetic") => phonetic(),
        Some("metaphone") => metaphone(),
        Some(_) | None => panic!("Cannot be none"),
    };

    match res {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description()),
    }
}
