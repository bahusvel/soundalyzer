extern crate midir;
#[macro_use]
extern crate phf;
#[macro_use]
extern crate lazy_static;

pub mod midi_backend;
pub mod midi_sync;

pub trait Backend {
    fn play_note(&mut self, note: u64);
}
