use std::f64::consts::PI;
use std::sync::atomic::AtomicPtr;
use std::error::Error;
use std::sync::Arc;
use std::time::{Instant, Duration};
use std::thread::sleep;
use std::mem::forget;


use spin::Mutex;
use portaudio as pa;

const CHANNELS: i32 = 1;
const NUM_SECONDS: i32 = 5;
const SAMPLE_RATE: f64 = 44_100.0;
const FRAMES_PER_BUFFER: u32 = 256;
const USECS_PER_SAMPLE: u32 = (1_000_000.0 / SAMPLE_RATE) as u32;
const DECAY: f32 = 2.0;

type Buffer = [f32; FRAMES_PER_BUFFER as usize];

const ZERO_BUFF: Buffer = [-1.0; FRAMES_PER_BUFFER as usize];

struct WriteBuffer {
    creation: Instant,
    buff: Buffer,
}

pub struct PortAudioBackend {
    write_buff: Arc<Mutex<WriteBuffer>>,
}

impl PortAudioBackend {
    pub fn new() -> Result<Self, Box<Error>> {
        let pa = try!(pa::PortAudio::new());
        let mut left_saw = 0.0;

        let mut settings = try!(pa.default_output_stream_settings(
            CHANNELS,
            SAMPLE_RATE,
            FRAMES_PER_BUFFER,
        ));
        // we won't output out of range samples so don't bother clipping them.
        settings.flags = pa::stream_flags::CLIP_OFF;

        // This routine will be called by the PortAudio engine when audio is needed. It may called at
        // interrupt level on some machines so don't do anything that could mess up the system like
        // dynamic resource allocation or IO.

        let play_buffer = Arc::new(Mutex::new(WriteBuffer {
            creation: Instant::now(),
            buff: [0.0; FRAMES_PER_BUFFER as usize],
        }));

        let backend = PortAudioBackend { write_buff: play_buffer.clone() };

        let callback = move |pa::OutputStreamCallbackArgs { buffer, frames, .. }| {
            let mut buff = play_buffer.lock();

            let mut carry = 0.0;
            for i in 0..buffer.len() {
                carry = (carry / DECAY) + buff.buff[i];
                buffer[i] = carry;
            }
            //buffer.copy_from_slice(&buff.buff);
            buff.buff.copy_from_slice(&ZERO_BUFF);
            buff.creation = Instant::now();
            pa::Continue
        };

        let mut stream = try!(pa.open_non_blocking_stream(settings, callback));
        try!(stream.start());

        // TODO Put these nicely into the backend struct
        forget(pa);
        forget(stream);

        Ok(backend)
    }

    pub fn play_now(&mut self, volume: f32) {
        let mut buff = self.write_buff.lock();
        let note_pos = (buff.creation.elapsed().subsec_micros() / USECS_PER_SAMPLE) as usize;
        if note_pos >= buff.buff.len() {
            //panic!("Sound thread is laggy");
            return;
        }
        buff.buff[note_pos] = volume;
    }
    fn dump_buff(&self) {
        let mut buff = self.write_buff.lock();
        println!("{:?}", &buff.buff[..]);
    }
}

// fn main() {
//     let mut backend = PortAudioBackend::new().expect("Failed to create backend");
//     for i in 0..440 {
//         for j in -1000..1000 {
//             backend.play_now(j as f32 / 1000.0);
//             for k in 0..100{()};
//         }
//         //backend.dump_buff();
//     }
// }
