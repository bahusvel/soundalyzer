#![feature(plugin)]
#![plugin(phf_macros)]

extern crate clap;
extern crate pcap;
extern crate soundalyzer;

use clap::{App, Arg};
use pcap::{Capture, Device};

use std::error::Error;
use std::io::{stdin, BufRead};

fn main() {
    let matches = App::new("Netalyzer")
        .version("0.0")
        .author("Denis Lavrov <bahus.vel@gmail.com>")
        .about("Like strace but outputs sounds")
        .arg(
            Arg::with_name("interface")
                .short("i")
                .takes_value(true)
                .required(true)
                .help("Network interface to audioalyze packets from"),
        )
        .arg(
            Arg::with_name("filter")
                .multiple(true)
                .takes_value(true)
                .help("Network interface to audioalyze packets from"),
        )
        .get_matches();

    let iface = matches.value_of("interface").unwrap();
    let filter = matches.values_of_lossy("filter").unwrap_or(Vec::new());

    let packets: Result<(), Box<Error>> = {
        move || {
            let mut cap = Capture::from_device(Device {
                name: iface.into(),
                desc: None,
            })?.promisc(true)
                .snaplen(0)
                .open()?;

            if filter.len() != 0 {
                cap.filter(&filter.join(" "));
            }

            while let Ok(packet) = cap.next() {
                println!("received packet! {:?}", packet);
            }
            Ok(())
        }
    }();

    match packets {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    }
}
