# Changelog

All notable changes to this project will be documented in this file.

The format is based on [Keep a Changelog](https://keepachangelog.com/en/1.0.0/),
and this project adheres to [Semantic Versioning](https://semver.org/spec/v2.0.0.html).

## [Unreleased]

## [0.1.0] - 2026-01-22

### Added
- F1 help menu overlay showing all keyboard shortcuts
- Full file path display in header (instead of just filename)
- User configuration support via `~/.config/clack/config.toml` for persistent preferences
- Configuration options: theme, typewriter_mode, focus_mode, sound_enabled, double_spacing
- Soft margin enforcement at column 72 (blocks typing past margin like authentic typewriter)
- Visual text wrapping at column 72 for consistent typewriter aesthetics
- Demo GIF showcasing Clack features in README

### Changed
- Binary name changed from `clack-rs` to `clack` for cleaner command-line usage
- Default save filename changed from `output.md` to `Untitled.md` to match header display
- Header "Clack" branding now uses reversed header colors for better readability across all themes
- Simplified footer to show only essential shortcuts (F1:Help, ^S:Save, Esc:Quit)
- Default theme changed from Dark to Paper for a warmer, more authentic typewriter experience
- All text now wraps visually at column 72 (bell_column) instead of window width
- Margin guide at column 72 now enforces soft wrap behavior (bell rings, typing blocked until Enter)

### Fixed
- Audio lag during fast typing by pre-decoding WAV files and implementing sink management
- "Clack" header was invisible in Light theme (black-on-black)
- Graceful handling when audio device is unavailable

### Added
- Distraction-free terminal typewriter experience with Rust and ratatui
- Typewriter mode (F3/Ctrl+T): keeps active line centered vertically on screen
- Focus mode (F2): dims inactive lines for enhanced concentration on current line
- Three visual themes (F5 to cycle):
  - Dark: Classic dark terminal theme
  - Paper: Cream paper on dark desk with authentic typewriter aesthetic
  - Retro: Amber CRT terminal emulation
- Authentic mechanical typewriter sounds:
  - Individual keystroke sounds
  - Space bar sound
  - Backspace sound
  - Carriage return on Enter
  - Bell at margin (column 72)
  - Page feed sound (every 54 lines)
- Sound toggle (F4) for silent writing sessions
- Double spacing toggle (F6) for manuscript-style formatting
- Markdown rendering support:
  - Bold (`**text**`)
  - Italic (`*text*`)
  - Headings (`# H1`, `## H2`, etc.)
  - Inline code (`` `code` ``)
  - Block quotes (`> quote`)
- Visual margin guide at bell column (configurable)
- Page break indicators with page numbers
- Real-time word and character count in status bar
- Current page indicator in footer
- Unsaved changes indicator (asterisk in header)
- File operations:
  - Open files via command line: `clack [filename]`
  - Save with Ctrl+S
  - Quit with Ctrl+Q
- Keyboard shortcuts:
  - Arrow keys for navigation
  - Home/End for line start/end
  - Ctrl+Left/Right for word-wise movement
  - Ctrl+Home/End for document start/end
  - PageUp/PageDown for page navigation
- Configurable layout settings (text width, padding, borders)
- Configurable typewriter settings (lines per page, bell column)
- Efficient text handling using ropey (Rope data structure)
- Low-latency audio playback using rodio
- Terminal UI built with ratatui and crossterm
