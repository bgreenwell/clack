use rodio::{OutputStream, Sink};
use std::sync::mpsc::{channel, Sender};
use std::thread;

// Define the types of sounds we can play
pub enum Sound {
    Key,
    Space,
    Backspace,
    Return,
    Ding,
    Startup,
    Toggle,
}

pub struct AudioEngine {
    tx: Sender<Sound>,
}

impl AudioEngine {
    pub fn new(enabled: bool) -> Self {
        let (tx, rx) = channel();

        if enabled {
            thread::spawn(move || {
                // Initialize the default audio output stream
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();

                // Play startup sound immediately if requested (we'll trigger it from App::new)
                // actually, we just wait for events.

                for sound_type in rx {
                    if let Ok(sink) = Sink::try_new(&stream_handle) {
                        let data: &[u8] = match sound_type {
                            Sound::Key => include_bytes!("assets/manual_key.wav").as_ref(),
                            Sound::Space => include_bytes!("assets/manual_space.wav").as_ref(),
                            Sound::Backspace => {
                                include_bytes!("assets/manual_backspace.wav").as_ref()
                            }
                            Sound::Return => include_bytes!("assets/manual_return.wav").as_ref(),
                            Sound::Ding => include_bytes!("assets/manual_bell.wav").as_ref(),
                            Sound::Startup => {
                                include_bytes!("assets/manual_load_long.wav").as_ref()
                            }
                            Sound::Toggle => include_bytes!("assets/manual_shift.wav").as_ref(),
                        };

                        let source = rodio::Decoder::new(std::io::Cursor::new(data)).unwrap();

                        sink.append(source);
                        sink.detach(); // Fire and forget
                    }
                }
            });
        }
        Self { tx }
    }

    pub fn trigger(&self, sound: Sound) {
        let _ = self.tx.send(sound);
    }
}
