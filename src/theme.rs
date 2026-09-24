use ratatui::style::{Color, Modifier, Style};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Brightness {
    Deep = 0,
    Dim = 1,
    Mid = 2,
    Head = 3,
}

impl Brightness {
    #[inline]
    pub fn from_dist(dist: usize, mid_end: usize, dim_end: usize) -> Self {
        if dist == 0 {
            Self::Head
        } else if dist < mid_end {
            Self::Mid
        } else if dist < dim_end {
            Self::Dim
        } else {
            Self::Deep
        }
    }
}

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq)]
pub enum Theme {
    #[default]
    Matrix,
    Pink,
    Gruvbox,
    Retro,
    Pastel,
    Dracula,
    Nord,
    Monokai,
    Tokyo,
    Solarized,
    OneDark,
    Catppuccin,
    RosePine,
    Everforest,
    Github,
}

impl Theme {
    pub fn next(self) -> Self {
        match self {
            Self::Matrix => Self::Pink,
            Self::Pink => Self::Gruvbox,
            Self::Gruvbox => Self::Retro,
            Self::Retro => Self::Pastel,
            Self::Pastel => Self::Dracula,
            Self::Dracula => Self::Nord,
            Self::Nord => Self::Monokai,
            Self::Monokai => Self::Tokyo,
            Self::Tokyo => Self::Solarized,
            Self::Solarized => Self::OneDark,
            Self::OneDark => Self::Catppuccin,
            Self::Catppuccin => Self::RosePine,
            Self::RosePine => Self::Everforest,
            Self::Everforest => Self::Github,
            Self::Github => Self::Matrix,
        }
    }

    pub fn styles(self) -> [Style; 4] {
        match self {
            Self::Matrix => [
                Style::default().fg(Color::Rgb(0, 70, 20)),
                Style::default().fg(Color::Rgb(0, 170, 60)),
                Style::default().fg(Color::Rgb(80, 255, 120)),
                Style::default()
                    .fg(Color::Rgb(230, 255, 230))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Pink => [
                Style::default().fg(Color::Rgb(90, 20, 70)),
                Style::default().fg(Color::Rgb(200, 70, 150)),
                Style::default().fg(Color::Rgb(255, 105, 180)),
                Style::default()
                    .fg(Color::Rgb(255, 240, 250))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Gruvbox => [
                Style::default().fg(Color::Rgb(80, 60, 30)),
                Style::default().fg(Color::Rgb(146, 131, 116)),
                Style::default().fg(Color::Rgb(214, 153, 33)),
                Style::default()
                    .fg(Color::Rgb(251, 241, 199))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Retro => [
                Style::default().fg(Color::Rgb(90, 60, 0)),
                Style::default().fg(Color::Rgb(180, 130, 0)),
                Style::default().fg(Color::Rgb(255, 220, 0)),
                Style::default()
                    .fg(Color::Rgb(255, 255, 200))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Pastel => [
                Style::default().fg(Color::Rgb(60, 45, 90)),
                Style::default().fg(Color::Rgb(120, 95, 210)),
                Style::default().fg(Color::Rgb(170, 150, 255)),
                Style::default()
                    .fg(Color::Rgb(240, 235, 255))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Dracula => [
                Style::default().fg(Color::Rgb(68, 71, 90)),
                Style::default().fg(Color::Rgb(98, 114, 164)),
                Style::default().fg(Color::Rgb(189, 147, 249)),
                Style::default()
                    .fg(Color::Rgb(248, 248, 242))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Nord => [
                Style::default().fg(Color::Rgb(46, 52, 64)),
                Style::default().fg(Color::Rgb(76, 86, 106)),
                Style::default().fg(Color::Rgb(136, 192, 208)),
                Style::default()
                    .fg(Color::Rgb(236, 239, 244))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Monokai => [
                Style::default().fg(Color::Rgb(73, 70, 52)),
                Style::default().fg(Color::Rgb(249, 38, 114)),
                Style::default().fg(Color::Rgb(230, 219, 116)),
                Style::default()
                    .fg(Color::Rgb(248, 248, 242))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Tokyo => [
                Style::default().fg(Color::Rgb(40, 52, 88)),
                Style::default().fg(Color::Rgb(65, 105, 184)),
                Style::default().fg(Color::Rgb(122, 162, 247)),
                Style::default()
                    .fg(Color::Rgb(192, 202, 245))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Solarized => [
                Style::default().fg(Color::Rgb(0, 43, 54)),
                Style::default().fg(Color::Rgb(88, 110, 117)),
                Style::default().fg(Color::Rgb(38, 139, 210)),
                Style::default()
                    .fg(Color::Rgb(253, 246, 227))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::OneDark => [
                Style::default().fg(Color::Rgb(40, 44, 52)),
                Style::default().fg(Color::Rgb(152, 195, 121)),
                Style::default().fg(Color::Rgb(97, 175, 239)),
                Style::default()
                    .fg(Color::Rgb(240, 244, 255))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Catppuccin => [
                Style::default().fg(Color::Rgb(30, 30, 46)),
                Style::default().fg(Color::Rgb(69, 71, 90)),
                Style::default().fg(Color::Rgb(203, 166, 247)),
                Style::default()
                    .fg(Color::Rgb(205, 214, 244))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::RosePine => [
                Style::default().fg(Color::Rgb(25, 23, 36)),
                Style::default().fg(Color::Rgb(110, 106, 134)),
                Style::default().fg(Color::Rgb(235, 188, 186)),
                Style::default()
                    .fg(Color::Rgb(224, 222, 244))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Everforest => [
                Style::default().fg(Color::Rgb(45, 53, 59)),
                Style::default().fg(Color::Rgb(122, 132, 120)),
                Style::default().fg(Color::Rgb(167, 192, 128)),
                Style::default()
                    .fg(Color::Rgb(211, 198, 170))
                    .add_modifier(Modifier::BOLD),
            ],
            Self::Github => [
                Style::default().fg(Color::Rgb(22, 27, 34)),
                Style::default().fg(Color::Rgb(48, 54, 61)),
                Style::default().fg(Color::Rgb(88, 166, 255)),
                Style::default()
                    .fg(Color::Rgb(201, 209, 217))
                    .add_modifier(Modifier::BOLD),
            ],
        }
    }
}
