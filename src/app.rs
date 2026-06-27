use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::raindrop::RainDrop;
use crate::theme::Theme;

const DROPS_PER_COL: usize = 3;
const DENSITY: f64 = 0.7;
const COLUMN_STEP: usize = 2;

pub struct App {
    pub drops: Vec<RainDrop>,
    pub width: u16,
    pub height: u16,
    theme: Theme,
    rng: SmallRng,
}

impl App {
    pub fn new(width: u16, height: u16) -> Self {
        let mut seed = rand::rng();
        let mut rng = SmallRng::from_rng(&mut seed);

        let drops = Self::spawn_drops(width, height, &mut rng);

        Self {
            drops,
            width,
            height,
            theme: Theme::Matrix,
            rng,
        }
    }

    pub fn tick(&mut self, dt: f32) {
        for drop in &mut self.drops {
            drop.step(dt, self.height, &mut self.rng);
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        if self.width == width && self.height == height {
            return;
        }

        self.width = width;
        self.height = height;

        self.drops = Self::spawn_drops(width, height, &mut self.rng);
    }

    pub fn next_theme(&mut self) {
        self.theme = self.theme.next();
    }

    pub fn theme(&self) -> Theme {
        self.theme
    }

    fn spawn_drops(width: u16, height: u16, rng: &mut SmallRng) -> Vec<RainDrop> {
        let mut drops = Vec::new();

        for col in (0..width).step_by(COLUMN_STEP) {
            if rng.random_bool(DENSITY) {
                for _ in 0..DROPS_PER_COL {
                    drops.push(RainDrop::new(col, height, rng));
                }
            }
        }

        drops
    }
}
