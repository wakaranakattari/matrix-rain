use ratatui::style::{Color, Modifier, Style};

#[derive(Clone, Copy)]
pub enum Theme {
    Matrix,
    Pink,
    Gruvbox,
    Retro,
    Pastel,
}

impl Theme {
    pub fn next(self) -> Self {
        match self {
            Self::Matrix => Self::Pink,
            Self::Pink => Self::Gruvbox,
            Self::Gruvbox => Self::Retro,
            Self::Retro => Self::Pastel,
            Self::Pastel => Self::Matrix,
        }
    }

    pub fn style(self, level: u8) -> Style {
        let (head, mid, dim, deep) = self.palette();

        match level {
            3 => head,
            2 => mid,
            1 => dim,
            _ => deep,
        }
    }

    fn palette(self) -> (Style, Style, Style, Style) {
        match self {
            Self::Matrix => (
                Style::default()
                    .fg(Color::Rgb(230, 255, 230))
                    .add_modifier(Modifier::BOLD),
                Style::default().fg(Color::Rgb(80, 255, 120)),
                Style::default().fg(Color::Rgb(0, 170, 60)),
                Style::default().fg(Color::Rgb(0, 70, 20)),
            ),
            Self::Pink => (
                Style::default()
                    .fg(Color::Rgb(255, 240, 250))
                    .add_modifier(Modifier::BOLD),
                Style::default().fg(Color::Rgb(255, 105, 180)),
                Style::default().fg(Color::Rgb(200, 70, 150)),
                Style::default().fg(Color::Rgb(90, 20, 70)),
            ),
            Self::Gruvbox => (
                Style::default()
                    .fg(Color::Rgb(251, 241, 199))
                    .add_modifier(Modifier::BOLD),
                Style::default().fg(Color::Rgb(214, 153, 33)),
                Style::default().fg(Color::Rgb(146, 131, 116)),
                Style::default().fg(Color::Rgb(80, 60, 30)),
            ),
            Self::Retro => (
                Style::default()
                    .fg(Color::Rgb(255, 255, 200))
                    .add_modifier(Modifier::BOLD),
                Style::default().fg(Color::Rgb(255, 220, 0)),
                Style::default().fg(Color::Rgb(180, 130, 0)),
                Style::default().fg(Color::Rgb(90, 60, 0)),
            ),
            Self::Pastel => (
                Style::default()
                    .fg(Color::Rgb(240, 235, 255))
                    .add_modifier(Modifier::BOLD),
                Style::default().fg(Color::Rgb(170, 150, 255)),
                Style::default().fg(Color::Rgb(120, 95, 210)),
                Style::default().fg(Color::Rgb(60, 45, 90)),
            ),
        }
    }
}
