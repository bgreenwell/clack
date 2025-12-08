use crate::app::App;
use crate::markdown;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &App) {
    let theme = &app.theme;

    // Set the overall background color
    let size = f.size();
    f.render_widget(
        Block::default().style(Style::default().bg(theme.base_bg)),
        size,
    );

    // 1. Split screen into Header, Body, Footer
    let vertical_chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length(1), // Header
            Constraint::Min(0),    // Body (Text Editor)
            Constraint::Length(1), // Footer
        ])
        .split(size);

    let header_area = vertical_chunks[0];
    let body_area = vertical_chunks[1];
    let footer_area = vertical_chunks[2];

    // --- HEADER ---
    let filename = app
        .file_path
        .as_ref()
        .and_then(|p| p.file_name())
        .and_then(|n| n.to_str())
        .unwrap_or("Untitled.md");

    let header_text = Line::from(vec![
        Span::styled(
            " Clack ",
            Style::default()
                .bg(theme.accent)
                .fg(Color::White)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(" {}", filename)),
    ]);
    f.render_widget(
        Paragraph::new(header_text).style(Style::default().bg(theme.header_bg).fg(theme.header_fg)),
        header_area,
    );

    // --- BODY (Text Area) ---
    let target_text_width = 80;
    let pad_left = 2;
    let pad_right = 2;
    let pad_top = 1;

    let paper_width = target_text_width + pad_left + pad_right + 2;

    // Center the paper horizontally
    let screen_width = body_area.width;
    let paper_x = if screen_width > paper_width {
        body_area.x + (screen_width - paper_width) / 2
    } else {
        body_area.x
    };

    let text_area = Rect {
        x: paper_x,
        y: body_area.y,
        width: paper_width,
        height: body_area.height,
    };

    let term_height = body_area.height as usize;
    let effective_width = if text_area.width >= (2 + pad_left + pad_right) {
        (text_area.width - 2 - pad_left - pad_right) as usize
    } else {
        1
    };

    // --- MANUAL WRAPPING & CURSOR MAPPING ---
    let (cursor_col, cursor_row) = app.get_cursor_position();
    let mut visual_lines: Vec<Line> = Vec::new();
    let mut visual_cursor_y = 0;
    let mut visual_cursor_x = 0;

    for (i, line) in app.content.lines().enumerate() {
        let parsed_line = markdown::parse_line(&line.to_string(), theme);

        // --- FOCUS MODE LOGIC ---
        let distance = i.abs_diff(cursor_row);
        let style_override = if app.focus_mode && distance > 0 {
            Some(
                Style::default()
                    .fg(theme.dim_text)
                    .add_modifier(Modifier::DIM),
            )
        } else {
            None
        };

        let raw_chars: Vec<(char, Style)> = parsed_line
            .spans
            .iter()
            .flat_map(|s| {
                let mut base_style = s.style;
                if base_style.fg.is_none() || base_style.fg == Some(Color::Reset) {
                    base_style = base_style.fg(theme.base_fg);
                }

                let final_style = if let Some(ov) = style_override {
                    base_style.fg(ov.fg.unwrap()).add_modifier(ov.add_modifier)
                } else {
                    base_style
                };
                s.content.chars().map(move |c| (c, final_style))
            })
            .collect();

        let mut current_spans = Vec::new();
        let mut width_counter = 0;
        let start_index = visual_lines.len();

        if raw_chars.is_empty() {
            visual_lines.push(Line::from(vec![]));
        } else {
            for (c, style) in raw_chars {
                if width_counter >= effective_width {
                    visual_lines.push(Line::from(current_spans));
                    current_spans = Vec::new();
                    width_counter = 0;
                }
                current_spans.push(Span::styled(c.to_string(), style));
                width_counter += 1;
            }
            if !current_spans.is_empty() {
                visual_lines.push(Line::from(current_spans));
            }
        }

        if i == cursor_row {
            let row_offset = cursor_col / effective_width;
            let col_offset = cursor_col % effective_width;
            let target_visual_row_idx = start_index + row_offset;

            if target_visual_row_idx >= visual_lines.len() {
                visual_lines.push(Line::from(vec![]));
            }

            visual_cursor_y = target_visual_row_idx;
            visual_cursor_x = col_offset;
        }
    }

    // --- SCROLLING LOGIC ---
    let inner_height = term_height.saturating_sub(2 + pad_top + 1);
    let center_line = inner_height / 2;

    let scroll_offset = if app.typewriter_mode {
        if visual_cursor_y > center_line {
            (visual_cursor_y - center_line) as u16
        } else {
            0
        }
    } else {
        let max_visible_row = inner_height.saturating_sub(1);
        if visual_cursor_y >= scroll_state_val(visual_cursor_y, inner_height) + inner_height {
            (visual_cursor_y - inner_height + 1) as u16
        } else if visual_cursor_y > max_visible_row {
            (visual_cursor_y - max_visible_row) as u16
        } else {
            0
        }
    };

    let paper_block = Block::default()
        .borders(Borders::LEFT | Borders::RIGHT)
        .padding(Padding::new(pad_left, pad_right, pad_top as u16, 1))
        .style(Style::default().fg(theme.border).bg(theme.paper_bg));

    let inner_text_area = paper_block.inner(text_area);

    let paragraph = Paragraph::new(visual_lines)
        .style(Style::default().fg(theme.base_fg).bg(theme.paper_bg))
        .scroll((scroll_offset, 0));

    f.render_widget(paper_block, text_area);
    f.render_widget(paragraph, inner_text_area);

    // --- CURSOR RENDERING ---
    let render_row = visual_cursor_y as i16 - scroll_offset as i16;
    let visual_y_start = text_area.y + pad_top as u16;
    let visual_x_start = text_area.x + 1 + pad_left;

    if render_row >= 0 && render_row < inner_height as i16 {
        f.set_cursor(
            visual_x_start + visual_cursor_x as u16,
            visual_y_start + render_row as u16,
        );
    }

    // --- FOOTER ---
    let mode_status = if app.typewriter_mode { "ON" } else { "OFF" };
    let focus_status = if app.focus_mode { "ON" } else { "OFF" };
    let sound_status = if app.sound_enabled { "ON" } else { "OFF" };
    let word_count = app.get_word_count();
    let char_count = app.get_char_count();

    let status_text = Line::from(vec![
        Span::styled(" TW: ", Style::default().fg(theme.header_fg)),
        Span::styled(
            mode_status,
            if app.typewriter_mode {
                Style::default().fg(theme.status_ok)
            } else {
                Style::default().fg(theme.status_bad)
            },
        ),
        Span::raw(" | "),
        Span::styled(" FOC: ", Style::default().fg(theme.header_fg)),
        Span::styled(
            focus_status,
            if app.focus_mode {
                Style::default().fg(theme.status_ok)
            } else {
                Style::default().fg(theme.status_bad)
            },
        ),
        Span::raw(" | "),
        Span::styled(" SND: ", Style::default().fg(theme.header_fg)),
        Span::styled(
            sound_status,
            if app.sound_enabled {
                Style::default().fg(theme.status_ok)
            } else {
                Style::default().fg(theme.status_bad)
            },
        ),
        Span::raw(" | "),
        Span::styled(
            format!("{} w / {} c", word_count, char_count),
            Style::default().fg(theme.header_fg),
        ),
        Span::raw(" | "),
        Span::styled(
            "F2:Foc F3:TW F4:Snd F5:Thm",
            Style::default().fg(theme.header_fg),
        ),
    ]);

    f.render_widget(
        Paragraph::new(status_text).style(Style::default().bg(theme.header_bg).fg(theme.header_fg)),
        footer_area,
    );
}

fn scroll_state_val(_y: usize, _h: usize) -> usize {
    0
}
