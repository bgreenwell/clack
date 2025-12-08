use ratatui::style::Color;

#[derive(Clone)]
pub struct Theme {
    pub _name: &'static str,
    pub base_bg: Color,
    pub base_fg: Color,
    pub paper_bg: Color,
    pub border: Color,
    pub header_bg: Color,
    pub header_fg: Color,
    pub accent: Color,     // For "Clack" text, active toggles
    pub dim_text: Color,   // For Focus Mode inactive lines
    pub status_ok: Color,  // Green usually
    pub status_bad: Color, // Red usually
}

impl Theme {
    pub fn dark() -> Self {
        Self {
            _name: "Dark",
            base_bg: Color::Reset, // Transparent/Black
            base_fg: Color::White,
            paper_bg: Color::Reset,
            border: Color::DarkGray,
            header_bg: Color::DarkGray,
            header_fg: Color::White,
            accent: Color::Blue,
            dim_text: Color::Rgb(50, 50, 50),
            status_ok: Color::Green,
            status_bad: Color::Red,
        }
    }

    pub fn light() -> Self {
        Self {
            _name: "Paper",
            // Dark Desk Background
            base_bg: Color::Rgb(30, 30, 30),
            base_fg: Color::Black,
            paper_bg: Color::Rgb(253, 246, 227),
            border: Color::Rgb(180, 170, 150),
            header_bg: Color::Rgb(238, 232, 213),
            header_fg: Color::Black,
            accent: Color::Rgb(38, 139, 210),    // Cyan/Blueish
            dim_text: Color::Rgb(200, 200, 190), // Dim version of black on cream
            status_ok: Color::Rgb(133, 153, 0),  // Olive Green
            status_bad: Color::Rgb(220, 50, 47), // Red
        }
    }

    pub fn retro() -> Self {
        let amber = Color::Rgb(255, 176, 0);
        let dim_amber = Color::Rgb(100, 70, 0);

        Self {
            _name: "Retro",
            base_bg: Color::Black,
            base_fg: amber,
            paper_bg: Color::Black,
            border: dim_amber,
            header_bg: Color::Rgb(40, 30, 0),
            header_fg: amber,
            accent: amber,
            dim_text: dim_amber,
            status_ok: amber,
            status_bad: Color::Red,
        }
    }
}

pub enum ThemeType {
    Dark,
    Light,
    Retro,
}

impl ThemeType {
    pub fn next(&self) -> Self {
        match self {
            ThemeType::Dark => ThemeType::Light,
            ThemeType::Light => ThemeType::Retro,
            ThemeType::Retro => ThemeType::Dark,
        }
    }
}
