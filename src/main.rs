mod app;
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
                Err(e) => eprintln!("Error loading file {}: {}", path.display(), e),
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
                KeyCode::Char('t') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    app.toggle_mode()
                }
                KeyCode::F(3) => app.toggle_mode(),
                KeyCode::F(2) => app.toggle_focus(),
                KeyCode::F(4) => app.toggle_sound(),
                KeyCode::F(5) => app.cycle_theme(),
                KeyCode::Char('s') if key.modifiers.contains(KeyModifiers::CONTROL) => {
                    match app.save_to_file() {
                        Ok(_) => { /* Maybe a brief status message? */ }
                        Err(e) => {
                            /* Handle error, e.g., display in status bar */
                            eprintln!("Error saving file: {}", e);
                        }
                    }
                }
                KeyCode::Enter => app.enter_key(),
                KeyCode::Char(c) => app.insert_char(c),
                KeyCode::Backspace => app.delete_char(),

                // Simple Navigation
                KeyCode::Left => {
                    if app.cursor_idx > 0 {
                        app.cursor_idx -= 1;
                    }
                }
                KeyCode::Right => {
                    if app.cursor_idx < app.content.len_chars() {
                        app.cursor_idx += 1;
                    }
                }
                KeyCode::Up => app.move_cursor_up(),
                KeyCode::Down => app.move_cursor_down(),

                _ => {}
            }
        }
    }
}
