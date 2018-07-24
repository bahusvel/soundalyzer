//! Play a sawtooth wave for several seconds.
//!
//! A rusty adaptation of the official PortAudio C "paex_saw.c" example by Phil Burk and Ross
//! Bencina.

extern crate portaudio;

use std::f64::consts::PI;

use portaudio as pa;

const CHANNELS: i32 = 1;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 10000;

static mut BUFFER: [f32; FRAMES_PER_BUFFER as usize] = [0.0; FRAMES_PER_BUFFER as usize];

fn main() {
    for i in 0..FRAMES_PER_BUFFER as usize {
        unsafe {
            BUFFER[i] = (i as f64 / 200 as f64 * PI * 2.0).sin() as f32;
        }
    }
    match run() {
        Ok(_) => {}
        e => {
            eprintln!("Example failed with the following: {:?}", e);
        }
    }
}

fn run() -> Result<(), pa::Error> {
    println!(
        "PortAudio Test: output sawtooth wave. SR = {}, BufSize = {}",
        SAMPLE_RATE, FRAMES_PER_BUFFER
    );

    let mut left_saw = 0.0;
    let mut right_saw = 0.0;

    let pa = try!(pa::PortAudio::new());

    let mut settings =
        try!(pa.default_output_stream_settings(CHANNELS, SAMPLE_RATE, FRAMES_PER_BUFFER));
    // we won't output out of range samples so don't bother clipping them.
    settings.flags = pa::stream_flags::CLIP_OFF;

    // This routine will be called by the PortAudio engine when audio is needed. It may called at
    // interrupt level on some machines so don't do anything that could mess up the system like
    // dynamic resource allocation or IO.
    let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
        buffer.clone_from_slice(unsafe { &BUFFER[..] });
        pa::Continue
    };

    let mut stream = try!(pa.open_non_blocking_stream(settings, callback));

    try!(stream.start());

    println!("Play for {} seconds.", NUM_SECONDS);
    pa.sleep(NUM_SECONDS * 1_000);

    try!(stream.stop());
    try!(stream.close());

    println!("Test finished.");

    Ok(())
}
