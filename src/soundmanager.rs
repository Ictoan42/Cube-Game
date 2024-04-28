
use std::io::Cursor;

use debug_ignore::DebugIgnore;
use rodio::{Decoder, OutputStreamHandle, Source};

#[derive(Debug)]
pub struct SoundManager {
    stream_handle: DebugIgnore<OutputStreamHandle>,
    sounds: Vec<(&'static[u8],f32)>,
}

impl SoundManager {
    pub fn new(sounds: Vec<(&'static[u8],f32)>, osh: OutputStreamHandle) -> Self {
        let stream_handle = osh;

        Self {
            stream_handle: stream_handle.into(),
            sounds,
        }
    }
    pub fn play(&mut self, soundindex: u32) {
        self.stream_handle.play_raw(
            Decoder::new_wav(
                Cursor::new(self.sounds[soundindex as usize].0)
            ).unwrap().convert_samples().amplify(self.sounds[soundindex as usize].1)
        ).unwrap();
    }
}
