Here is the complete implementation of **`clack`**, consolidated into a single document for easy reference.

# Clack: The Distraction-Free Terminal Typewriter

**Clack** is a Rust-based CLI text editor designed for "flow state" writing. It features a "Typewriter Mode" (keeping the active line centered) and mechanical keyboard sound effects to provide tactile feedback.

## 1\. Project Setup

Create a new Rust project:

```bash
cargo new clack
cd clack
```

-----

## 2\. Dependencies (`Cargo.toml`)

Update your `Cargo.toml` file to include the necessary libraries for TUI rendering, text management, and audio.

```toml
[package]
name = "clack"
version = "0.1.0"
edition = "2021"

[dependencies]
ratatui = "0.26"      # The TUI framework
crossterm = "0.27"    # Terminal event handling
ropey = "1.6"         # Efficient text buffer (Rope data structure)
rodio = "0.17"        # Audio playback
anyhow = "1.0"        # Error handling
```

*(Note: On Linux, ensure you have ALSA development headers installed: `sudo apt install libasound2-dev`)*

-----

## 3\. The Source Code

Create the following files inside the `src/` directory.

### `src/sound.rs` (The Audio Engine)

This module spawns a background thread to handle audio triggers so the editor never lags while typing.

```rust
use std::sync::mpsc::{channel, Sender};
use std::thread;
use rodio::{OutputStream, Sink};

// Define the types of sounds we can play
pub enum Sound {
    Click,
    Ding,
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
                // 'unwrap' is used here for simplicity; in prod, handle errors gracefully
                let (_stream, stream_handle) = OutputStream::try_default().unwrap();
                
                for _sound in rx {
                    // Logic to play sound. 
                    // To enable real sound, uncomment the block below and ensure 
                    // you have 'click.wav' and 'ding.wav' in your assets folder.
                    
                    /*
                    let sink = Sink::try_new(&stream_handle).unwrap();
                    let data = match _sound {
                        Sound::Click => include_bytes!("../assets/click.wav").as_ref(),
                        Sound::Ding => include_bytes!("../assets/ding.wav").as_ref(),
                    };
                    // Create a cursor to treat the embedded bytes like a file
                    let source = rodio::Decoder::new(std::io::Cursor::new(data)).unwrap();
                    
                    sink.append(source);
                    sink.detach(); // Fire and forget; don't wait for sound to finish
                    */
                }
            });
        }
        Self { tx }
    }

    pub fn trigger(&self, sound: Sound) {
        // Sending to the channel is non-blocking
        let _ = self.tx.send(sound);
    }
}
```

### `src/app.rs` (State Management)

This module manages the text buffer (`Rope`), cursor position, and configuration state.

```rust
use ropey::Rope;
use crate::sound::{AudioEngine, Sound};

pub struct App {
    pub content: Rope,
    pub cursor_idx: usize,      // Absolute character index in the text
    pub typewriter_mode: bool,  // Toggle for vertical centering
    pub audio: AudioEngine,
}

impl App {
    pub fn new() -> Self {
        Self {
            content: Rope::from_str("# Clack\n\nStart typing... (F3 toggles Typewriter Mode)"),
            cursor_idx: 0,
            typewriter_mode: false, // Default to standard mode
            audio: AudioEngine::new(true),
        }
    }

    pub fn toggle_mode(&mut self) {
        self.typewriter_mode = !self.typewriter_mode;
    }

    pub fn insert_char(&mut self, c: char) {
        self.content.insert_char(self.cursor_idx, c);
        self.cursor_idx += 1;
        self.audio.trigger(Sound::Click);
    }

    pub fn delete_char(&mut self) {
        if self.cursor_idx > 0 {
            self.content.remove(self.cursor_idx - 1..self.cursor_idx);
            self.cursor_idx -= 1;
            self.audio.trigger(Sound::Click);
        }
    }
    
    pub fn enter_key(&mut self) {
        self.content.insert_char(self.cursor_idx, '\n');
        self.cursor_idx += 1;
        self.audio.trigger(Sound::Ding);
    }

    // Helper to calculate (col, row) for the UI from the flat index
    pub fn get_cursor_position(&self) -> (usize, usize) {
        let row = self.content.char_to_line(self.cursor_idx);
        let row_start_idx = self.content.line_to_char(row);
        let col = self.cursor_idx - row_start_idx;
        (col, row)
    }
}
```

### `src/ui.rs` (Rendering & Typewriter Logic)

This module handles the TUI layout, Markdown highlighting, and the "Typewriter Scroll" math.

```rust
use ratatui::{
    backend::Backend,
    layout::{Constraint, Direction, Layout},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Paragraph},
    Frame,
};
use crate::app::App;

pub fn draw<B: Backend>(f: &mut Frame<B>, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([Constraint::Min(0)])
        .split(f.size());

    // --- TYPEWRITER MATH ---
    let (_, cursor_y) = app.get_cursor_position();
    let term_height = chunks[0].height as usize;
    
    let scroll_offset = if app.typewriter_mode {
        // In Typewriter mode, we want the cursor to stay at the visual center.
        let target_center = term_height / 2;
        if cursor_y > target_center {
            (cursor_y - target_center) as u16
        } else {
            0
        }
    } else {
        // Standard mode: simplistic scrolling (always show top 0 for now)
        // In a full implementation, you'd track a viewport state here.
        if cursor_y as u16 > term_height as u16 {
             (cursor_y as u16 - term_height as u16)
        } else {
            0
        }
    };

    // --- PARSING & HIGHLIGHTING ---
    let text: Vec<Line> = app.content.lines().map(|line| {
        let raw = line.to_string();
        let trimmed = raw.trim_end(); // Remove newline for display
        
        if trimmed.starts_with("# ") {
            Line::from(Span::styled(trimmed, Style::default().fg(Color::Blue).add_modifier(Modifier::BOLD)))
        } else if trimmed.starts_with("## ") {
            Line::from(Span::styled(trimmed, Style::default().fg(Color::Cyan).add_modifier(Modifier::BOLD)))
        } else {
            Line::from(trimmed)
        }
    }).collect();

    // --- LAYOUT (Margins) ---
    // We constrain the text to the middle 60% of the screen for readability
    let layout = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Percentage(20), // Left Margin
            Constraint::Percentage(60), // Writing Area
            Constraint::Percentage(20), // Right Margin
        ])
        .split(chunks[0]);

    let paragraph = Paragraph::new(text)
        .scroll((scroll_offset, 0)); // Apply the calculated scroll

    f.render_widget(paragraph, layout[1]);

    // --- CURSOR RENDERING ---
    let (cursor_col, cursor_row) = app.get_cursor_position();
    
    // Calculate where to draw the cursor relative to the scroll
    let visual_row = cursor_row as i16 - scroll_offset as i16;
    
    // Only draw cursor if it falls within the visible screen area
    if visual_row >= 0 && visual_row < term_height as i16 {
        f.set_cursor(
            layout[1].x + cursor_col as u16,
            layout[1].y + visual_row as u16,
        );
    }
}
```

### `src/main.rs` (The Entry Point)

This orchestrates the terminal setup and event loop.

```rust
mod app;
mod ui;
mod sound;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use std::io;

fn main() -> anyhow::Result<()> {
    // 1. Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Initialize App
    let mut app = App::new();
    
    // 3. Run Event Loop
    let res = run_app(&mut terminal, &mut app);

    // 4. Teardown (Restore terminal even if app crashes)
    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{:?}", err)
    }

    Ok(())
}

fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui::draw(f, app))?;

        if let Event::Key(key) = event::read()? {
            match key.code {
                KeyCode::Esc => return Ok(()),
                KeyCode::F(3) => app.toggle_mode(),
                KeyCode::Enter => app.enter_key(),
                KeyCode::Char(c) => app.insert_char(c),
                KeyCode::Backspace => app.delete_char(),
                
                // Simple Navigation
                KeyCode::Left => { if app.cursor_idx > 0 { app.cursor_idx -= 1; } }
                KeyCode::Right => { if app.cursor_idx < app.content.len_chars() { app.cursor_idx += 1; } }
                
                _ => {}
            }
        }
    }
}
```

-----

## 4\. Running the Project

1.  **Build and Run:**
    ```bash
    cargo run
    ```
2.  **Usage:**
      * **Type** naturally.
      * Press **`F3`** to toggle Typewriter Mode (try it after writing 10+ lines).
      * Press **`Esc`** to quit.

## 5\. Next Steps

  * **Enable Sound:** Place `.wav` files in a `src/assets/` folder and uncomment the logic in `src/sound.rs`.
  * **Saving Files:** Add logic to `main.rs` to write `app.content.to_string()` to a file upon exit.
  * **Theme:** Adjust the colors in `src/ui.rs` to match your preferred palette (e.g., Gruvbox or Solarized).
