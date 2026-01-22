use serde::{Deserialize, Serialize};
use std::fs;
use std::path::PathBuf;

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

/// User preferences for default application behavior
#[derive(Debug, Serialize, Deserialize)]
pub struct UserPreferences {
    /// Default theme: "Paper", "Dark", or "Retro"
    #[serde(default = "default_theme")]
    pub theme: String,

    /// Enable typewriter mode by default (keeps active line centered)
    #[serde(default = "default_true")]
    pub typewriter_mode: bool,

    /// Enable focus mode by default (dims inactive lines)
    #[serde(default)]
    pub focus_mode: bool,

    /// Enable sound effects by default
    #[serde(default = "default_true")]
    pub sound_enabled: bool,

    /// Enable double spacing by default
    #[serde(default)]
    pub double_spacing: bool,
}

fn default_theme() -> String {
    "Paper".to_string()
}

fn default_true() -> bool {
    true
}

impl Default for UserPreferences {
    fn default() -> Self {
        Self {
            theme: default_theme(),
            typewriter_mode: true,
            focus_mode: false,
            sound_enabled: true,
            double_spacing: false,
        }
    }
}

impl UserPreferences {
    /// Parse theme string into ThemeType
    pub fn parse_theme(&self) -> crate::theme::ThemeType {
        match self.theme.to_lowercase().as_str() {
            "dark" => crate::theme::ThemeType::Dark,
            "paper" | "light" => crate::theme::ThemeType::Light,
            "retro" => crate::theme::ThemeType::Retro,
            _ => crate::theme::ThemeType::Light, // Default to Paper
        }
    }

    /// Load user preferences from config file, or return defaults if not found
    pub fn load() -> Self {
        let config_path = Self::config_path();

        if let Some(path) = config_path {
            if path.exists() {
                if let Ok(contents) = fs::read_to_string(&path) {
                    if let Ok(prefs) = toml::from_str(&contents) {
                        return prefs;
                    }
                }
            }
        }

        // Return defaults if config doesn't exist or can't be read
        Self::default()
    }

    /// Get the config file path: ~/.config/clack/config.toml
    fn config_path() -> Option<PathBuf> {
        let home = std::env::var("HOME").ok()?;
        let config_dir = PathBuf::from(home).join(".config").join("clack");
        Some(config_dir.join("config.toml"))
    }

    /// Save current preferences to config file
    #[allow(dead_code)]
    pub fn save(&self) -> std::io::Result<()> {
        if let Some(path) = Self::config_path() {
            // Create config directory if it doesn't exist
            if let Some(parent) = path.parent() {
                fs::create_dir_all(parent)?;
            }

            let toml_string = toml::to_string_pretty(self).map_err(std::io::Error::other)?;
            fs::write(&path, toml_string)?;
        }
        Ok(())
    }
}
