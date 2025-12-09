mod app;
mod config;
mod markdown;
mod sound;
mod theme;
mod ui;

use app::App;
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, KeyModifiers},
    execute,
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{backend::CrosstermBackend, Terminal};
use sound::Sound;
use std::io;
use std::path::PathBuf;

fn main() -> anyhow::Result<()> {
    // 1. Setup Terminal
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // 2. Initialize App
    let args: Vec<String> = std::env::args().collect();
    let mut app = App::new();
    if let Some(file_arg) = args.get(1) {
        let path = PathBuf::from(file_arg);
        if path.exists() && path.is_file() {
            match std::fs::read_to_string(&path) {
                Ok(content) => {
                    app.content = ropey::Rope::from_str(&content);
                    app.file_path = Some(path.clone());
                    app.cursor_idx = 0; // Reset cursor for new file
                }
                Err(e) => {
                    app.set_error(format!("Failed to load {}: {e}", path.display()));
                }
            }
        } else {
            app.file_path = Some(path.clone());
            app.content = ropey::Rope::new();
        }
    }

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
        println!("{err:?}")
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
                KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.toggle_mode()
                }
                KeyCode::F(3) => app.toggle_mode(),
                KeyCode::F(2) => app.toggle_focus(),
                KeyCode::F(4) => app.toggle_sound(),
                KeyCode::F(5) => app.cycle_theme(),
                KeyCode::F(6) => app.toggle_double_spacing(),
                KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    if let Err(e) = app.save_to_file() {
                        app.set_error(format!("Failed to save: {e}"));
                    }
                }
                KeyCode::Enter => {
                    app.clear_status();
                    app.enter_key();
                    // Check if we crossed a page boundary and need to pause for feed sound
                    if app.check_and_play_page_feed() {
                        // Brief pause to let the mechanical "feed" action feel real
                        std::thread::sleep(std::time::Duration::from_millis(
                            app.config.typewriter.page_feed_pause_ms,
                        ));
                    }
                }
                KeyCode::Char(c) => {
                    app.clear_status();
                    app.insert_char(c);
                    // Check if we crossed a page boundary while typing
                    if app.check_and_play_page_feed() {
                        std::thread::sleep(std::time::Duration::from_millis(
                            app.config.typewriter.page_feed_pause_ms,
                        ));
                    }
                }
                KeyCode::Backspace => {
                    app.clear_status();
                    app.delete_char();
                }
                KeyCode::Delete => {
                    app.clear_status();
                    app.delete_char_forward();
                }

                // Simple Navigation
                KeyCode::Left => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        app.move_word_left();
                    } else if app.cursor_idx > 0 {
                        app.cursor_idx -= 1;
                    }
                }
                KeyCode::Right => {
                    if key.modifiers.contains(KeyModifiers::CONTROL) {
                        app.move_word_right();
                    } else if app.cursor_idx < app.content.len_chars() {
                        app.cursor_idx += 1;
                    }
                }
                KeyCode::Up => app.move_cursor_up(),
                KeyCode::Down => {
                    let crossed_page = app.move_cursor_down();
                    // Play sound and pause if we crossed a page boundary
                    if crossed_page && app.sound_enabled {
                        app.audio.trigger(Sound::Feed);
                        std::thread::sleep(std::time::Duration::from_millis(
                            app.config.typewriter.page_feed_pause_ms,
                        ));
                    }
                }
                KeyCode::Home => app.move_to_line_start(),
                KeyCode::End => app.move_to_line_end(),

                _ => {}
            }
        }
    }
}
