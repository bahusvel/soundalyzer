#![feature(plugin)]
#![plugin(phf_macros)]

extern crate clap;
extern crate phf;
extern crate soundalyzer;

mod metaphone;

use clap::{App, Arg};

use soundalyzer::midi_sync::MIDISync;
use soundalyzer::Backend;

use std::error::Error;
use std::io::{stdin, BufRead};

type line_to_note = fn(&str) -> u64;

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

    match metaphone() {
        Ok(_) => (),
        Err(err) => println!("Error: {}", err.description()),
    }
}
