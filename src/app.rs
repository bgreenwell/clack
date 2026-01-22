use crate::config::{Config, UserPreferences};
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
    pub double_spacing: bool,  // Toggle for double spacing between lines
    pub show_help: bool,       // Toggle for help overlay
    pub audio: AudioEngine,
    pub file_path: Option<PathBuf>,
    pub current_theme_type: ThemeType,
    pub theme: Theme,
    pub status_message: Option<String>, // Status message for user feedback
    pub config: Config,                 // Application configuration
    pub has_unsaved_changes: bool,      // Track if there are unsaved modifications
    pub last_page_number: usize,        // Track current page for feed sound
    cached_word_count: Option<usize>,   // Cached word count for performance
    cached_char_count: Option<usize>,   // Cached character count for performance
}

impl App {
    pub fn new() -> Self {
        // Load user preferences from config file
        let prefs = UserPreferences::load();
        let theme_type = prefs.parse_theme();
        let theme = match theme_type {
            ThemeType::Dark => Theme::dark(),
            ThemeType::Light => Theme::light(),
            ThemeType::Retro => Theme::retro(),
        };

        let app = Self {
            content: Rope::new(),
            cursor_idx: 0,
            typewriter_mode: prefs.typewriter_mode,
            focus_mode: prefs.focus_mode,
            sound_enabled: prefs.sound_enabled,
            double_spacing: prefs.double_spacing,
            show_help: false,
            audio: AudioEngine::new(prefs.sound_enabled),
            file_path: None,
            current_theme_type: theme_type,
            theme,
            status_message: None,
            config: Config::new(),
            has_unsaved_changes: false,
            last_page_number: 1,
            cached_word_count: None,
            cached_char_count: None,
        };

        if app.sound_enabled {
            app.audio.trigger(Sound::Startup);
        }

        app
    }

    pub fn save_to_file(&mut self) -> io::Result<()> {
        let path = match &self.file_path {
            Some(p) => p.clone(),
            None => {
                let default_path = PathBuf::from("Untitled.md");
                self.file_path = Some(default_path.clone());
                default_path
            }
        };

        let mut file = fs::File::create(&path)?;
        for chunk in self.content.chunks() {
            file.write_all(chunk.as_bytes())?;
        }

        self.has_unsaved_changes = false;
        self.status_message = Some(format!("Saved to {}", path.display()));
        Ok(())
    }

    pub fn set_error(&mut self, message: String) {
        self.status_message = Some(format!("Error: {message}"));
    }

    pub fn clear_status(&mut self) {
        self.status_message = None;
    }

    fn invalidate_count_cache(&mut self) {
        self.cached_word_count = None;
        self.cached_char_count = None;
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

    pub fn toggle_double_spacing(&mut self) {
        self.double_spacing = !self.double_spacing;
        if self.sound_enabled {
            self.audio.trigger(Sound::Toggle);
        }
    }

    pub fn toggle_help(&mut self) {
        self.show_help = !self.show_help;
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
        // Check margin before inserting character
        let (_col, row) = self.get_cursor_position();
        let line_len = self.content.line(row).len_chars();

        // Soft margin: prevent typing past bell_column (like a real typewriter margin stop)
        if line_len >= self.config.typewriter.bell_column {
            // Play bell to indicate margin reached
            if self.sound_enabled {
                self.audio.trigger(Sound::Ding);
            }
            return; // Don't insert the character
        }

        // Insert the character
        self.content.insert_char(self.cursor_idx, c);
        self.cursor_idx += 1;
        self.has_unsaved_changes = true;
        self.invalidate_count_cache();

        if self.sound_enabled {
            if c == ' ' {
                self.audio.trigger(Sound::Space);
            } else {
                self.audio.trigger(Sound::Key);
            }

            // Bell warning when approaching margin
            let new_line_len = self.content.line(row).len_chars();
            if new_line_len == self.config.typewriter.bell_column {
                self.audio.trigger(Sound::Ding);
            }
        }
    }

    pub fn delete_char(&mut self) {
        if self.cursor_idx > 0 {
            self.content.remove(self.cursor_idx - 1..self.cursor_idx);
            self.cursor_idx -= 1;
            self.has_unsaved_changes = true;
            self.invalidate_count_cache();
            if self.sound_enabled {
                self.audio.trigger(Sound::Backspace);
            }
        }
    }

    pub fn delete_char_forward(&mut self) {
        if self.cursor_idx < self.content.len_chars() {
            self.content.remove(self.cursor_idx..self.cursor_idx + 1);
            self.has_unsaved_changes = true;
            self.invalidate_count_cache();
            if self.sound_enabled {
                self.audio.trigger(Sound::Backspace);
            }
        }
    }

    pub fn enter_key(&mut self) {
        self.content.insert_char(self.cursor_idx, '\n');
        self.cursor_idx += 1;
        self.has_unsaved_changes = true;
        self.invalidate_count_cache();
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

    pub fn get_char_count(&mut self) -> usize {
        if let Some(count) = self.cached_char_count {
            count
        } else {
            let count = self.content.len_chars();
            self.cached_char_count = Some(count);
            count
        }
    }

    pub fn get_current_page(&self) -> usize {
        let (_col, row) = self.get_cursor_position();
        (row / self.config.typewriter.lines_per_page) + 1
    }

    pub fn check_and_play_page_feed(&mut self) -> bool {
        let current_page = self.get_current_page();
        if current_page > self.last_page_number {
            self.last_page_number = current_page;
            if self.sound_enabled {
                self.audio.trigger(Sound::Feed);
            }
            true // Indicates we crossed a page boundary
        } else {
            false
        }
    }

    pub fn get_word_count(&mut self) -> usize {
        if let Some(count) = self.cached_word_count {
            count
        } else {
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

            self.cached_word_count = Some(words);
            words
        }
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

    pub fn move_cursor_down(&mut self) -> bool {
        let (col, row) = self.get_cursor_position();
        if row < self.content.len_lines() - 1 {
            let old_page = (row / self.config.typewriter.lines_per_page) + 1;
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

            // Check if we crossed a page boundary
            let new_page = (new_row / self.config.typewriter.lines_per_page) + 1;
            if new_page > old_page {
                // Update last_page_number if we've moved to a new highest page
                if new_page > self.last_page_number {
                    self.last_page_number = new_page;
                }
                return true; // Crossed page boundary going down
            }
        }
        false
    }

    pub fn move_to_line_start(&mut self) {
        let (_col, row) = self.get_cursor_position();
        self.cursor_idx = self.content.line_to_char(row);
    }

    pub fn move_to_line_end(&mut self) {
        let (_col, row) = self.get_cursor_position();
        let line_start = self.content.line_to_char(row);
        let line_len = self.content.line(row).len_chars();

        // Don't include the newline character in the end position
        let line_end = if line_len > 0 {
            let last_char_idx = line_start + line_len - 1;
            if self.content.char(last_char_idx) == '\n' {
                last_char_idx
            } else {
                line_start + line_len
            }
        } else {
            line_start
        };

        self.cursor_idx = line_end;
    }

    pub fn move_word_left(&mut self) {
        if self.cursor_idx == 0 {
            return;
        }

        let mut idx = self.cursor_idx - 1;

        // Skip whitespace
        while idx > 0 && self.content.char(idx).is_whitespace() {
            idx -= 1;
        }

        // Move to start of word
        while idx > 0 && !self.content.char(idx - 1).is_whitespace() {
            idx -= 1;
        }

        self.cursor_idx = idx;
    }

    pub fn move_word_right(&mut self) {
        let max_idx = self.content.len_chars();
        if self.cursor_idx >= max_idx {
            return;
        }

        let mut idx = self.cursor_idx;

        // Skip current word
        while idx < max_idx && !self.content.char(idx).is_whitespace() {
            idx += 1;
        }

        // Skip whitespace
        while idx < max_idx && self.content.char(idx).is_whitespace() {
            idx += 1;
        }

        self.cursor_idx = idx;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_new_app_has_empty_content() {
        let app = App::new();
        assert_eq!(app.content.len_chars(), 0);
        assert_eq!(app.cursor_idx, 0);
        assert!(!app.has_unsaved_changes);
    }

    #[test]
    fn test_insert_char_increases_cursor() {
        let mut app = App::new();
        app.insert_char('a');
        assert_eq!(app.cursor_idx, 1);
        assert_eq!(app.content.len_chars(), 1);
        assert!(app.has_unsaved_changes);
    }

    #[test]
    fn test_insert_multiple_chars() {
        let mut app = App::new();
        app.insert_char('h');
        app.insert_char('i');
        assert_eq!(app.cursor_idx, 2);
        assert_eq!(app.content.to_string(), "hi");
    }

    #[test]
    fn test_delete_char_decreases_cursor() {
        let mut app = App::new();
        app.insert_char('a');
        app.insert_char('b');
        app.delete_char();
        assert_eq!(app.cursor_idx, 1);
        assert_eq!(app.content.to_string(), "a");
    }

    #[test]
    fn test_delete_char_at_start_does_nothing() {
        let mut app = App::new();
        app.delete_char();
        assert_eq!(app.cursor_idx, 0);
        assert_eq!(app.content.len_chars(), 0);
    }

    #[test]
    fn test_enter_key_adds_newline() {
        let mut app = App::new();
        app.insert_char('a');
        app.enter_key();
        app.insert_char('b');
        assert_eq!(app.content.to_string(), "a\nb");
        assert_eq!(app.cursor_idx, 3);
    }

    #[test]
    fn test_get_cursor_position_single_line() {
        let mut app = App::new();
        app.insert_char('h');
        app.insert_char('i');
        let (col, row) = app.get_cursor_position();
        assert_eq!(row, 0);
        assert_eq!(col, 2);
    }

    #[test]
    fn test_get_cursor_position_multiline() {
        let mut app = App::new();
        app.insert_char('a');
        app.enter_key();
        app.insert_char('b');
        app.insert_char('c');
        let (col, row) = app.get_cursor_position();
        assert_eq!(row, 1);
        assert_eq!(col, 2);
    }

    #[test]
    fn test_word_count_empty() {
        let mut app = App::new();
        assert_eq!(app.get_word_count(), 0);
    }

    #[test]
    fn test_word_count_single_word() {
        let mut app = App::new();
        for c in "hello".chars() {
            app.insert_char(c);
        }
        assert_eq!(app.get_word_count(), 1);
    }

    #[test]
    fn test_word_count_multiple_words() {
        let mut app = App::new();
        for c in "hello world test".chars() {
            app.insert_char(c);
        }
        assert_eq!(app.get_word_count(), 3);
    }

    #[test]
    fn test_char_count() {
        let mut app = App::new();
        for c in "hello".chars() {
            app.insert_char(c);
        }
        assert_eq!(app.get_char_count(), 5);
    }

    #[test]
    fn test_unsaved_changes_tracking() {
        let mut app = App::new();
        assert!(!app.has_unsaved_changes);

        app.insert_char('a');
        assert!(app.has_unsaved_changes);

        // Simulate a successful save
        app.has_unsaved_changes = false;
        assert!(!app.has_unsaved_changes);

        app.delete_char();
        assert!(app.has_unsaved_changes);
    }

    #[test]
    fn test_toggle_mode() {
        let mut app = App::new();
        assert!(app.typewriter_mode); // Now defaults to true
        app.toggle_mode();
        assert!(!app.typewriter_mode);
        app.toggle_mode();
        assert!(app.typewriter_mode);
    }

    #[test]
    fn test_toggle_focus() {
        let mut app = App::new();
        assert!(!app.focus_mode);
        app.toggle_focus();
        assert!(app.focus_mode);
        app.toggle_focus();
        assert!(!app.focus_mode);
    }

    #[test]
    fn test_clear_status() {
        let mut app = App::new();
        app.set_error("test error".to_string());
        assert!(app.status_message.is_some());
        app.clear_status();
        assert!(app.status_message.is_none());
    }

    #[test]
    fn test_move_cursor_up() {
        let mut app = App::new();
        app.insert_char('a');
        app.enter_key();
        app.insert_char('b');

        let initial_idx = app.cursor_idx;
        app.move_cursor_up();
        assert!(app.cursor_idx < initial_idx);
    }

    #[test]
    fn test_move_cursor_down() {
        let mut app = App::new();
        app.insert_char('a');
        app.enter_key();
        app.insert_char('b');

        // Move up first
        app.move_cursor_up();
        let idx_after_up = app.cursor_idx;

        // Then move down
        app.move_cursor_down();
        assert!(app.cursor_idx > idx_after_up);
    }

    #[test]
    fn test_move_to_line_start() {
        let mut app = App::new();
        for c in "hello world".chars() {
            app.insert_char(c);
        }
        // Cursor is at end of line
        assert_eq!(app.cursor_idx, 11);

        app.move_to_line_start();
        assert_eq!(app.cursor_idx, 0);
    }

    #[test]
    fn test_move_to_line_end() {
        let mut app = App::new();
        for c in "hello world".chars() {
            app.insert_char(c);
        }
        app.move_to_line_start();
        assert_eq!(app.cursor_idx, 0);

        app.move_to_line_end();
        assert_eq!(app.cursor_idx, 11);
    }

    #[test]
    fn test_move_word_right() {
        let mut app = App::new();
        for c in "hello world test".chars() {
            app.insert_char(c);
        }
        app.move_to_line_start();

        app.move_word_right();
        // Should be at start of "world" (after "hello ")
        assert_eq!(app.cursor_idx, 6);

        app.move_word_right();
        // Should be at start of "test"
        assert_eq!(app.cursor_idx, 12);
    }

    #[test]
    fn test_move_word_left() {
        let mut app = App::new();
        for c in "hello world test".chars() {
            app.insert_char(c);
        }
        // Cursor at end

        app.move_word_left();
        // Should be at start of "test"
        assert_eq!(app.cursor_idx, 12);

        app.move_word_left();
        // Should be at start of "world"
        assert_eq!(app.cursor_idx, 6);

        app.move_word_left();
        // Should be at start of "hello"
        assert_eq!(app.cursor_idx, 0);
    }

    #[test]
    fn test_delete_char_forward() {
        let mut app = App::new();
        for c in "abc".chars() {
            app.insert_char(c);
        }
        app.move_to_line_start();

        app.delete_char_forward();
        assert_eq!(app.content.to_string(), "bc");
        assert_eq!(app.cursor_idx, 0);
        assert!(app.has_unsaved_changes);
    }

    #[test]
    fn test_delete_forward_at_end() {
        let mut app = App::new();
        app.insert_char('a');
        // Cursor at end
        app.delete_char_forward();
        // Should do nothing
        assert_eq!(app.content.to_string(), "a");
    }

    #[test]
    fn test_get_current_page_empty() {
        let app = App::new();
        assert_eq!(app.get_current_page(), 1);
    }

    #[test]
    fn test_get_current_page_single_page() {
        let mut app = App::new();
        // Add 10 lines (well below 54), cursor at end
        for _ in 0..10 {
            app.enter_key();
        }
        // Cursor is on line 10, which is page 1
        assert_eq!(app.get_current_page(), 1);
    }

    #[test]
    fn test_get_current_page_multiple_pages() {
        let mut app = App::new();
        // Add 54 lines, cursor ends up on line 54 (page 1)
        for _ in 0..54 {
            app.enter_key();
        }
        // Line 54 is still on page 1 (0-53 = page 1, 54-107 = page 2)
        assert_eq!(app.get_current_page(), 2);

        // Add another 54 lines
        for _ in 0..54 {
            app.enter_key();
        }
        // Now on line 108, which is page 3
        assert_eq!(app.get_current_page(), 3);
    }

    #[test]
    fn test_page_follows_cursor() {
        let mut app = App::new();
        // Create 100 lines
        for _ in 0..100 {
            app.enter_key();
        }
        // Cursor is at line 100, which is page 2 (54-107)
        assert_eq!(app.get_current_page(), 2);

        // Move cursor to beginning
        app.cursor_idx = 0;
        assert_eq!(app.get_current_page(), 1);

        // Move to line 54 (start of page 2)
        app.cursor_idx = app.content.line_to_char(54);
        assert_eq!(app.get_current_page(), 2);
    }

    #[test]
    fn test_page_feed_detection() {
        let mut app = App::new();
        // Should be on page 1
        assert_eq!(app.last_page_number, 1);

        // Add lines up to page boundary
        for _ in 0..53 {
            app.enter_key();
            assert!(!app.check_and_play_page_feed()); // Still on page 1
        }

        // Cross to page 2
        app.enter_key();
        assert!(app.check_and_play_page_feed()); // Should trigger feed
        assert_eq!(app.last_page_number, 2);

        // Subsequent checks shouldn't trigger again
        assert!(!app.check_and_play_page_feed());
    }
}
