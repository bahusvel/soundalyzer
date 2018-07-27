extern crate midir;
#[macro_use]
extern crate phf;
#[macro_use]
extern crate lazy_static;
extern crate spin;
extern crate portaudio;

pub mod midi_backend;
pub mod midi_sync;
pub mod portaudio_backend;

pub trait Backend {
    fn play_note(&mut self, note: u64);
}
