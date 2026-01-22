use crate::app::App;
use crate::markdown;
use ratatui::{
    layout::{Constraint, Direction, Layout, Rect},
    style::{Color, Modifier, Style},
    text::{Line, Span},
    widgets::{Block, Borders, Padding, Paragraph},
    Frame,
};

pub fn draw(f: &mut Frame, app: &mut App) {
    // Get cached counts before borrowing theme (to avoid borrow checker issues)
    let word_count = app.get_word_count();
    let char_count = app.get_char_count();

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
    let filepath = app
        .file_path
        .as_ref()
        .map(|p| p.display().to_string())
        .unwrap_or_else(|| "Untitled.md".to_string());

    let unsaved_indicator = if app.has_unsaved_changes { " *" } else { "" };

    let header_text = Line::from(vec![
        Span::styled(
            " Clack ",
            Style::default()
                .bg(theme.header_fg)
                .fg(theme.header_bg)
                .add_modifier(Modifier::BOLD),
        ),
        Span::raw(format!(" {filepath}{unsaved_indicator}")),
    ]);
    f.render_widget(
        Paragraph::new(header_text).style(Style::default().bg(theme.header_bg).fg(theme.header_fg)),
        header_area,
    );

    // --- BODY (Text Area) ---
    let target_text_width = app.config.layout.text_width;
    let pad_left = app.config.layout.pad_left;
    let pad_right = app.config.layout.pad_right;
    let pad_top = app.config.layout.pad_top;

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

        // Add blank line for double spacing if enabled
        if app.double_spacing {
            visual_lines.push(Line::from(vec![]));
        }

        // Insert page break AFTER the last line of each page
        // Check if the CURRENT line (i) is the last line of a page
        if (i + 1) % app.config.typewriter.lines_per_page == 0 {
            let next_page_number = ((i + 1) / app.config.typewriter.lines_per_page) + 1;
            let label = format!(" Page {next_page_number} ");
            let label_len = label.len();

            // Calculate padding for centering
            let total_width = effective_width;
            let remaining_width = total_width.saturating_sub(label_len);
            let left_padding = remaining_width / 2;
            let right_padding = remaining_width - left_padding;

            // Create centered separator: "─── Page X ───"
            let left_bar = "─".repeat(left_padding);
            let right_bar = "─".repeat(right_padding);
            let separator_line = format!("{left_bar}{label}{right_bar}");

            // Add blank line before, separator, and blank line after
            visual_lines.push(Line::from(vec![])); // Blank line before
            visual_lines.push(Line::from(vec![Span::styled(
                separator_line,
                Style::default().fg(theme.guide_color),
            )]));
            visual_lines.push(Line::from(vec![])); // Blank line after
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
    let inner_height = term_height.saturating_sub(
        2 + app.config.layout.pad_top as usize + app.config.layout.pad_bottom as usize,
    );
    let center_line = inner_height / 2;

    let scroll_offset = if app.typewriter_mode {
        if visual_cursor_y > center_line {
            (visual_cursor_y - center_line) as u16
        } else {
            0
        }
    } else {
        let max_visible_row = inner_height.saturating_sub(1);
        if visual_cursor_y > max_visible_row {
            (visual_cursor_y - max_visible_row) as u16
        } else {
            0
        }
    };

    let border_type = if app.config.layout.fancy_borders {
        ratatui::widgets::BorderType::Double
    } else {
        ratatui::widgets::BorderType::Plain
    };

    let paper_block = Block::default()
        .borders(Borders::ALL)
        .border_type(border_type)
        .padding(Padding::new(
            pad_left,
            pad_right,
            pad_top,
            app.config.layout.pad_bottom,
        ))
        .style(Style::default().fg(theme.border).bg(theme.paper_bg));

    let inner_text_area = paper_block.inner(text_area);

    // --- CURSOR RENDERING PREP ---
    let render_row = visual_cursor_y as i16 - scroll_offset as i16;
    let cursor_visual_y_start = text_area.y + 1 + app.config.layout.pad_top; // +1 for block's top border
    let cursor_visual_x_start = text_area.x + 1 + app.config.layout.pad_left;

    // Get character at cursor position BEFORE moving visual_lines
    let char_at_cursor = if visual_cursor_y < visual_lines.len() {
        visual_lines[visual_cursor_y]
            .spans
            .get(visual_cursor_x)
            .and_then(|span| span.content.chars().next())
            .unwrap_or(' ')
    } else {
        ' '
    };

    let paragraph = Paragraph::new(visual_lines)
        .style(Style::default().fg(theme.base_fg).bg(theme.paper_bg))
        .scroll((scroll_offset, 0));

    f.render_widget(paper_block, text_area);
    f.render_widget(paragraph, inner_text_area);

    // --- MARGIN GUIDE ---
    if app.config.layout.show_margin_guide {
        let margin_col = app.config.typewriter.bell_column;
        if margin_col < effective_width {
            let guide_x = cursor_visual_x_start + margin_col as u16;
            let guide_style = Style::default().fg(theme.guide_color);

            // Draw a subtle vertical line at the margin
            for row in 0..inner_height {
                let y = cursor_visual_y_start + row as u16;
                if y < text_area.y + text_area.height {
                    f.render_widget(
                        Paragraph::new("┊").style(guide_style),
                        Rect {
                            x: guide_x,
                            y,
                            width: 1,
                            height: 1,
                        },
                    );
                }
            }
        }
    }

    // --- BLOCK CURSOR RENDERING ---
    if render_row >= 0 && render_row < inner_height as i16 {
        let cursor_x = cursor_visual_x_start + visual_cursor_x as u16;
        let cursor_y = cursor_visual_y_start + render_row as u16;

        // Render block cursor with inverted colors
        let cursor_block = Paragraph::new(char_at_cursor.to_string()).style(
            Style::default()
                .bg(theme.base_fg) // Invert: foreground color as background
                .fg(theme.paper_bg), // Invert: paper color as foreground
        );

        f.render_widget(
            cursor_block,
            Rect {
                x: cursor_x,
                y: cursor_y,
                width: 1,
                height: 1,
            },
        );
    }

    // --- FOOTER ---
    let status_text = if let Some(ref msg) = app.status_message {
        // Show status message if present
        Line::from(vec![Span::styled(
            format!(" {msg}"),
            if msg.starts_with("Error:") {
                Style::default().fg(theme.status_bad)
            } else {
                Style::default().fg(theme.status_ok)
            },
        )])
    } else {
        // Show normal status bar
        let mode_status = if app.typewriter_mode { "ON" } else { "OFF" };
        let focus_status = if app.focus_mode { "ON" } else { "OFF" };
        let sound_status = if app.sound_enabled { "ON" } else { "OFF" };
        let double_spacing_status = if app.double_spacing { "ON" } else { "OFF" };
        let current_page = app.get_current_page();

        Line::from(vec![
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
            Span::styled(" 2X: ", Style::default().fg(theme.header_fg)),
            Span::styled(
                double_spacing_status,
                if app.double_spacing {
                    Style::default().fg(theme.status_ok)
                } else {
                    Style::default().fg(theme.status_bad)
                },
            ),
            Span::raw(" | "),
            Span::styled(
                format!("Page {current_page}"),
                Style::default()
                    .fg(theme.accent)
                    .add_modifier(Modifier::BOLD),
            ),
            Span::raw(" | "),
            Span::styled(
                format!("{word_count} w / {char_count} c"),
                Style::default().fg(theme.header_fg),
            ),
            Span::raw(" | "),
            Span::styled(
                "F1:Help ^S:Save Esc:Quit",
                Style::default().fg(theme.header_fg),
            ),
        ])
    };

    f.render_widget(
        Paragraph::new(status_text).style(Style::default().bg(theme.header_bg).fg(theme.header_fg)),
        footer_area,
    );

    // --- HELP OVERLAY ---
    if app.show_help {
        draw_help_overlay(f, theme);
    }
}

fn draw_help_overlay(f: &mut Frame, theme: &crate::theme::Theme) {
    let size = f.size();

    // Create centered modal dimensions
    let modal_width = 60;
    let modal_height = 18;
    let modal_x = (size.width.saturating_sub(modal_width)) / 2;
    let modal_y = (size.height.saturating_sub(modal_height)) / 2;

    let modal_area = Rect {
        x: modal_x,
        y: modal_y,
        width: modal_width,
        height: modal_height,
    };

    // Create help content
    let help_text = vec![
        Line::from(vec![Span::styled(
            "                    CLACK - HELP                    ",
            Style::default()
                .fg(theme.base_bg)
                .bg(theme.accent)
                .add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![Span::styled(
            "  Keyboard shortcuts",
            Style::default().add_modifier(Modifier::BOLD),
        )]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  F1", Style::default().fg(theme.accent)),
            Span::raw("      Toggle this help menu"),
        ]),
        Line::from(vec![
            Span::styled("  F2", Style::default().fg(theme.accent)),
            Span::raw("      Toggle focus mode (dim inactive lines)"),
        ]),
        Line::from(vec![
            Span::styled("  F3", Style::default().fg(theme.accent)),
            Span::raw("      Toggle typewriter mode (center line)"),
        ]),
        Line::from(vec![
            Span::styled("  F4", Style::default().fg(theme.accent)),
            Span::raw("      Toggle sound effects"),
        ]),
        Line::from(vec![
            Span::styled("  F5", Style::default().fg(theme.accent)),
            Span::raw("      Cycle theme (Light/Dark/Retro)"),
        ]),
        Line::from(vec![
            Span::styled("  F6", Style::default().fg(theme.accent)),
            Span::raw("      Toggle double spacing"),
        ]),
        Line::from(""),
        Line::from(vec![
            Span::styled("  Ctrl+S", Style::default().fg(theme.accent)),
            Span::raw("  Save file"),
        ]),
        Line::from(vec![
            Span::styled("  Ctrl+T", Style::default().fg(theme.accent)),
            Span::raw("  Toggle typewriter mode"),
        ]),
        Line::from(vec![
            Span::styled("  Esc", Style::default().fg(theme.accent)),
            Span::raw("     Quit application"),
        ]),
    ];

    // Render the modal with border
    let block = Block::default()
        .borders(Borders::ALL)
        .border_style(Style::default().fg(theme.border))
        .style(Style::default().bg(theme.paper_bg).fg(theme.base_fg));

    f.render_widget(block, modal_area);

    // Render text inside the modal
    let inner_area = Rect {
        x: modal_area.x + 1,
        y: modal_area.y + 1,
        width: modal_area.width.saturating_sub(2),
        height: modal_area.height.saturating_sub(2),
    };

    let paragraph =
        Paragraph::new(help_text).style(Style::default().bg(theme.paper_bg).fg(theme.base_fg));

    f.render_widget(paragraph, inner_area);
}
