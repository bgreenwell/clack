use crate::sound::{AudioEngine, Sound};
use crate::theme::{Theme, ThemeType};
use ropey::Rope;
use std::fs;
use std::io::{self, Write};
use std::path::PathBuf;

pub struct App {
    pub content: Rope,
    pub cursor_idx: usize,     // Absolute character index in the text
    pub typewriter_mode: bool, // Toggle for vertical centering
    pub focus_mode: bool,      // Toggle for dimming inactive lines
    pub sound_enabled: bool,   // Toggle for sound effects
    pub audio: AudioEngine,
    pub file_path: Option<PathBuf>,
    pub current_theme_type: ThemeType,
    pub theme: Theme,
}

impl App {
    pub fn new() -> Self {
        let app = Self {
            content: Rope::new(),
            cursor_idx: 0,
            typewriter_mode: false,
            focus_mode: false,
            sound_enabled: true,
            audio: AudioEngine::new(true),
            file_path: None,
            current_theme_type: ThemeType::Dark,
            theme: Theme::dark(),
        };

        if app.sound_enabled {
            app.audio.trigger(Sound::Startup);
        }

        app
    }

    pub fn save_to_file(&self) -> io::Result<()> {
        let path = match &self.file_path {
            Some(p) => p.clone(),
            None => PathBuf::from("output.md"),
        };

        let mut file = fs::File::create(&path)?;
        for chunk in self.content.chunks() {
            file.write_all(chunk.as_bytes())?;
        }
        Ok(())
    }

    pub fn toggle_mode(&mut self) {
        self.typewriter_mode = !self.typewriter_mode;
        if self.sound_enabled {
            self.audio.trigger(Sound::Toggle);
        }
    }

    pub fn toggle_focus(&mut self) {
        self.focus_mode = !self.focus_mode;
        if self.sound_enabled {
            self.audio.trigger(Sound::Toggle);
        }
    }

    pub fn toggle_sound(&mut self) {
        self.sound_enabled = !self.sound_enabled;
        if self.sound_enabled {
            self.audio.trigger(Sound::Toggle);
        }
    }

    pub fn cycle_theme(&mut self) {
        self.current_theme_type = self.current_theme_type.next();
        self.theme = match self.current_theme_type {
            ThemeType::Dark => Theme::dark(),
            ThemeType::Light => Theme::light(),
            ThemeType::Retro => Theme::retro(),
        };
        if self.sound_enabled {
            self.audio.trigger(Sound::Toggle);
        }
    }

    pub fn insert_char(&mut self, c: char) {
        self.content.insert_char(self.cursor_idx, c);
        self.cursor_idx += 1;

        if self.sound_enabled {
            if c == ' ' {
                self.audio.trigger(Sound::Space);
            } else {
                self.audio.trigger(Sound::Key);
            }

            // Bell Logic
            let (_col, row) = self.get_cursor_position();
            let line_len = self.content.line(row).len_chars();

            if line_len == 72 {
                self.audio.trigger(Sound::Ding);
            }
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_idx > 0 {
            self.content.remove(self.cursor_idx - 1..self.cursor_idx);
            self.cursor_idx -= 1;
            if self.sound_enabled {
                self.audio.trigger(Sound::Backspace);
            }
        }
    }

    pub fn enter_key(&mut self) {
        self.content.insert_char(self.cursor_idx, '\n');
        self.cursor_idx += 1;
        if self.sound_enabled {
            self.audio.trigger(Sound::Return);
        }
    }

    pub fn get_cursor_position(&self) -> (usize, usize) {
        let row = self.content.char_to_line(self.cursor_idx);
        let row_start_idx = self.content.line_to_char(row);
        let col = self.cursor_idx - row_start_idx;
        (col, row)
    }

    pub fn get_char_count(&self) -> usize {
        self.content.len_chars()
    }

    pub fn get_word_count(&self) -> usize {
        // Simple approximation: count spaces + newlines?
        // Or iterate chunks and split.
        // For a large document, iterating all chars is expensive every frame.
        // But for a text editor, <1MB is fine.
        // Ropey's iterator is fast.

        let mut words = 0;
        let mut in_word = false;

        for chunk in self.content.chunks() {
            for c in chunk.chars() {
                if c.is_whitespace() {
                    if in_word {
                        words += 1;
                        in_word = false;
                    }
                } else {
                    in_word = true;
                }
            }
        }
        if in_word {
            words += 1;
        }
        words
    }

    pub fn move_cursor_up(&mut self) {
        let (col, row) = self.get_cursor_position();
        if row > 0 {
            let new_row = row - 1;
            let new_row_start = self.content.line_to_char(new_row);
            let new_row_len = self.content.line(new_row).len_chars();

            let new_line_char_count = if new_row_len > 0 {
                let last_char = self.content.char(new_row_start + new_row_len - 1);
                if last_char == '\n' {
                    new_row_len - 1
                } else {
                    new_row_len
                }
            } else {
                0
            };

            let new_col = col.min(new_line_char_count);
            self.cursor_idx = new_row_start + new_col;
        }
    }

    pub fn move_cursor_down(&mut self) {
        let (col, row) = self.get_cursor_position();
        if row < self.content.len_lines() - 1 {
            let new_row = row + 1;
            let new_row_start = self.content.line_to_char(new_row);
            let new_row_len = self.content.line(new_row).len_chars();

            let new_line_char_count = if new_row_len > 0 {
                let last_char = self.content.char(new_row_start + new_row_len - 1);
                if last_char == '\n' {
                    new_row_len - 1
                } else {
                    new_row_len
                }
            } else {
                0
            };

            let new_col = col.min(new_line_char_count);
            self.cursor_idx = new_row_start + new_col;
        }
    }
}
