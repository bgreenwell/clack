use rodio::{source::Buffered, Decoder, OutputStream, Sink, Source};
use std::io::Cursor;
use std::sync::mpsc::{channel, Sender};
use std::thread;

// Type alias for our buffered audio source
type BufferedSource = Buffered<Decoder<Cursor<&'static [u8]>>>;

// Define the types of sounds we can play
pub enum Sound {
    Key,
    Space,
    Backspace,
    Return,
    Ding,
    Startup,
    Toggle,
    Feed, // Paper feed sound for page breaks
}

// Pre-decoded audio sources for low-latency playback
struct AudioSources {
    key: BufferedSource,
    space: BufferedSource,
    backspace: BufferedSource,
    return_key: BufferedSource,
    ding: BufferedSource,
    startup: BufferedSource,
    toggle: BufferedSource,
    feed: BufferedSource,
}

impl AudioSources {
    fn new() -> Self {
        // Helper function to decode and buffer a sound
        fn load_sound(data: &'static [u8]) -> BufferedSource {
            Decoder::new(Cursor::new(data)).unwrap().buffered()
        }

        Self {
            key: load_sound(include_bytes!("assets/manual_key.wav")),
            space: load_sound(include_bytes!("assets/manual_space.wav")),
            backspace: load_sound(include_bytes!("assets/manual_backspace.wav")),
            return_key: load_sound(include_bytes!("assets/manual_return.wav")),
            ding: load_sound(include_bytes!("assets/manual_bell.wav")),
            startup: load_sound(include_bytes!("assets/manual_load_long.wav")),
            toggle: load_sound(include_bytes!("assets/manual_shift.wav")),
            feed: load_sound(include_bytes!("assets/manual_feed.wav")),
        }
    }

    fn get(&self, sound: &Sound) -> BufferedSource {
        match sound {
            Sound::Key => self.key.clone(),
            Sound::Space => self.space.clone(),
            Sound::Backspace => self.backspace.clone(),
            Sound::Return => self.return_key.clone(),
            Sound::Ding => self.ding.clone(),
            Sound::Startup => self.startup.clone(),
            Sound::Toggle => self.toggle.clone(),
            Sound::Feed => self.feed.clone(),
        }
    }
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
                let (_stream, stream_handle) = match OutputStream::try_default() {
                    Ok((stream, handle)) => (stream, handle),
                    Err(e) => {
                        eprintln!("Failed to initialize audio output stream: {e}. No sound will be played.");
                        return;
                    }
                };

                // Pre-decode all audio sources at startup
                let sources = AudioSources::new();

                // Keep a collection of active sinks
                // We'll maintain up to POOL_SIZE concurrent sounds
                const POOL_SIZE: usize = 10;
                let mut active_sinks: Vec<Sink> = Vec::with_capacity(POOL_SIZE);

                for sound_type in rx {
                    // Clean up finished sinks (ones that are empty)
                    active_sinks.retain(|sink| !sink.empty());

                    // If we have room for more sounds, play it
                    if active_sinks.len() < POOL_SIZE {
                        if let Ok(sink) = Sink::try_new(&stream_handle) {
                            // Get the pre-decoded buffered source
                            let source = sources.get(&sound_type);
                            sink.append(source);
                            active_sinks.push(sink);
                        }
                    }
                    // If we're at capacity, drop the sound to avoid lag
                    // This prevents creating too many sinks during rapid typing
                }
            });
        }
        Self { tx }
    }

    pub fn trigger(&self, sound: Sound) {
        let _ = self.tx.send(sound);
    }
}
