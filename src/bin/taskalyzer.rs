#![feature(plugin)]
#![plugin(phf_macros)]
#![feature(libc)]

extern crate clap;
extern crate libc;
extern crate nix;
extern crate phf;
extern crate soundalyzer;

mod syscallmap;

use clap::{App, Arg};
use nix::sys::ptrace;
use nix::sys::wait::{waitpid, WaitStatus};
use nix::unistd::Pid;
use std::ptr;

use soundalyzer::midi_backend::MIDIBackend;
use soundalyzer::Backend;

use std::error::Error;
use std::io::{stdin, BufRead};
use std::process::Command;

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

    let trace: Result<(), Box<Error>> = {
        move || {
            let child = Command::new("sh").arg("-c").arg("echo hi").spawn()?;
            let pid = Pid::from_raw(child.id() as i32);
            ptrace::attach(pid)?;
            let mut in_call = false;
            loop {
                let status = waitpid(pid, None)?;
                match status {
                    WaitStatus::PtraceSyscall(pid) => {
                        in_call = !in_call;
                        let a_reg = unsafe {
                            ptrace::ptrace(
                                ptrace::Request::PTRACE_PEEKUSER,
                                pid,
                                (8 * libc::RAX) as *mut nix::libc::c_void,
                                ptr::null_mut(),
                            )
                        }?;
                        ptrace::syscall(pid)?;
                    }
                    WaitStatus::Exited(_, _) => return Ok(()),
                    exit => panic!("Unknown exit occured"),
                }
            }
            Ok(())
        }
    }();

    match trace {
        Ok(_) => (),
        Err(err) => println!("Error: {:?}", err),
    }
}
