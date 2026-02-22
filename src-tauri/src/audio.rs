use std::io::Cursor;
use std::thread;

use rodio::{OutputStreamBuilder, play};

const WORK_SOUND: &[u8] = include_bytes!("../resources/sounds/work_complete.wav");
const BREAK_SOUND: &[u8] = include_bytes!("../resources/sounds/break_complete.wav");

fn play_sound(bytes: &'static [u8]) {
    thread::spawn(move || {
        if let Ok(stream) = OutputStreamBuilder::open_default_stream() {
            let cursor = Cursor::new(bytes);
            if let Ok(sink) = play(&stream.mixer(), cursor) {
                sink.sleep_until_end();
            }
        }
    });
}

pub fn play_work_complete_sound() {
    play_sound(WORK_SOUND);
}

pub fn play_break_complete_sound() {
    play_sound(BREAK_SOUND);
}
