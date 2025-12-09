use crate::theme::Theme;
use ratatui::{
    style::{Modifier, Style},
    text::{Line, Span},
};

pub fn parse_line(line: &str, theme: &Theme) -> Line<'static> {
    let raw = line.trim_end();

    let mut spans = Vec::new();
    let mut current_text = String::new();
    let mut chars = raw.chars().peekable();

    let dim_style = Style::default().fg(theme.guide_color);

    // Headers first - similar to bold/italic with dimmed markup
    if let Some(header_text) = raw.strip_prefix("## ") {
        spans.push(Span::styled("## ", dim_style));
        spans.push(Span::styled(
            header_text.to_string(),
            Style::default()
                .fg(theme.base_fg)
                .add_modifier(Modifier::BOLD),
        ));
        return Line::from(spans);
    }

    if let Some(header_text) = raw.strip_prefix("# ") {
        spans.push(Span::styled("# ", dim_style));
        spans.push(Span::styled(
            header_text.to_string(),
            Style::default()
                .fg(theme.base_fg)
                .add_modifier(Modifier::BOLD),
        ));
        return Line::from(spans);
    }

    // Reset spans for body text parsing

    while let Some(c) = chars.next() {
        match c {
            '*' => {
                if let Some(&next_c) = chars.peek() {
                    if next_c == '*' {
                        chars.next();
                        if !current_text.is_empty() {
                            spans.push(Span::raw(current_text.clone()));
                            current_text.clear();
                        }

                        let mut bold_text = String::new();
                        let mut closed = false;

                        while let Some(bc) = chars.next() {
                            if bc == '*' {
                                if let Some(&next_bc) = chars.peek() {
                                    if next_bc == '*' {
                                        chars.next();
                                        closed = true;
                                        break;
                                    }
                                }
                            }
                            bold_text.push(bc);
                        }

                        if closed {
                            spans.push(Span::styled("**", dim_style));
                            spans.push(Span::styled(
                                bold_text,
                                Style::default()
                                    .add_modifier(Modifier::BOLD)
                                    .fg(theme.base_fg), // Ensure bold text uses base_fg
                            ));
                            spans.push(Span::styled("**", dim_style));
                        } else {
                            spans.push(Span::raw("**"));
                            spans.push(Span::raw(bold_text));
                        }
                        continue;
                    }
                }

                if !current_text.is_empty() {
                    spans.push(Span::raw(current_text.clone()));
                    current_text.clear();
                }

                let mut italic_text = String::new();
                let mut closed = false;

                for ic in chars.by_ref() {
                    if ic == '*' {
                        closed = true;
                        break;
                    }
                    italic_text.push(ic);
                }

                if closed {
                    spans.push(Span::styled("*", dim_style));
                    spans.push(Span::styled(
                        italic_text,
                        Style::default()
                            .add_modifier(Modifier::ITALIC)
                            .fg(theme.base_fg), // Ensure italic text uses base_fg
                    ));
                    spans.push(Span::styled("*", dim_style));
                } else {
                    spans.push(Span::raw("*"));
                    spans.push(Span::raw(italic_text));
                }
            }
            '_' => {
                if let Some(&next_c) = chars.peek() {
                    if next_c == '_' {
                        chars.next();
                        if !current_text.is_empty() {
                            spans.push(Span::raw(current_text.clone()));
                            current_text.clear();
                        }

                        let mut bold_text = String::new();
                        let mut closed = false;

                        while let Some(bc) = chars.next() {
                            if bc == '_' {
                                if let Some(&next_bc) = chars.peek() {
                                    if next_bc == '_' {
                                        chars.next();
                                        closed = true;
                                        break;
                                    }
                                }
                            }
                            bold_text.push(bc);
                        }

                        if closed {
                            spans.push(Span::styled("__", dim_style));
                            spans.push(Span::styled(
                                bold_text,
                                Style::default()
                                    .add_modifier(Modifier::BOLD)
                                    .fg(theme.base_fg),
                            ));
                            spans.push(Span::styled("__", dim_style));
                        } else {
                            spans.push(Span::raw("__"));
                            spans.push(Span::raw(bold_text));
                        }
                        continue;
                    }
                }

                if !current_text.is_empty() {
                    spans.push(Span::raw(current_text.clone()));
                    current_text.clear();
                }

                let mut italic_text = String::new();
                let mut closed = false;

                for ic in chars.by_ref() {
                    if ic == '_' {
                        closed = true;
                        break;
                    }
                    italic_text.push(ic);
                }

                if closed {
                    spans.push(Span::styled("_", dim_style));
                    spans.push(Span::styled(
                        italic_text,
                        Style::default()
                            .add_modifier(Modifier::ITALIC)
                            .fg(theme.base_fg),
                    ));
                    spans.push(Span::styled("_", dim_style));
                } else {
                    spans.push(Span::raw("_"));
                    spans.push(Span::raw(italic_text));
                }
            }
            _ => {
                current_text.push(c);
            }
        }
    }

    if !current_text.is_empty() {
        spans.push(Span::raw(current_text));
    }

    Line::from(spans)
}
