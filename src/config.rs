/// Configuration constants for the Clack typewriter application
///
/// Text area and layout configuration
pub struct LayoutConfig {
    /// Target width for the text area in characters
    pub text_width: u16,
    /// Left padding inside the paper border
    pub pad_left: u16,
    /// Right padding inside the paper border
    pub pad_right: u16,
    /// Top padding inside the paper border
    pub pad_top: u16,
    /// Bottom padding inside the paper border
    pub pad_bottom: u16,
    /// Show margin guide at bell column
    pub show_margin_guide: bool,
    /// Use fancy borders for paper effect
    pub fancy_borders: bool,
}

impl Default for LayoutConfig {
    fn default() -> Self {
        Self {
            text_width: 80,
            pad_left: 2,
            pad_right: 2,
            pad_top: 1,
            pad_bottom: 1,
            show_margin_guide: true,
            fancy_borders: true,
        }
    }
}

/// Typewriter behavior configuration
pub struct TypewriterConfig {
    /// Column position at which the bell should ring (margin warning)
    pub bell_column: usize,
    /// Number of lines per page (triggers paper feed sound)
    pub lines_per_page: usize,
    /// Pause duration in milliseconds when feeding new page
    pub page_feed_pause_ms: u64,
}

impl Default for TypewriterConfig {
    fn default() -> Self {
        Self {
            bell_column: 72,
            lines_per_page: 54,
            page_feed_pause_ms: 350, // Just long enough to feel the "mechanical" action
        }
    }
}

/// Application configuration
#[derive(Default)]
pub struct Config {
    pub layout: LayoutConfig,
    pub typewriter: TypewriterConfig,
}

impl Config {
    pub fn new() -> Self {
        Self::default()
    }
}
