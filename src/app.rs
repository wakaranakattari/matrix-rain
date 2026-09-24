use rand::{Rng, SeedableRng, rngs::SmallRng};

use crate::raindrop::RainDrop;
use crate::theme::Theme;

const DROPS_PER_COL: usize = 3;
const DENSITY: f64 = 0.7;
const COLUMN_STEP: usize = 2;

pub struct App {
    drops: Vec<RainDrop>,
    width: u16,
    height: u16,
    theme: Theme,
    rng: SmallRng,
}

impl App {
    pub fn new(width: u16, height: u16) -> Self {
        let mut rng = SmallRng::from_os_rng();
        let drops = Self::spawn_drops(width, height, &mut rng);

        Self {
            drops,
            width,
            height,
            theme: Theme::default(),
            rng,
        }
    }

    #[inline]
    pub fn drops(&self) -> &[RainDrop] {
        &self.drops
    }

    #[inline]
    pub fn theme(&self) -> Theme {
        self.theme
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

        if width < self.width {
            self.drops.retain(|drop| drop.col() < width);
        } else if width > self.width {
            let old_width = self.width;
            let new_cols = (width - old_width) as usize / COLUMN_STEP + 1;
            self.drops.reserve(new_cols * DROPS_PER_COL);

            let drops = &mut self.drops;
            let rng = &mut self.rng;
            for col in (0..width).step_by(COLUMN_STEP) {
                if col >= old_width {
                    Self::push_column(drops, col, height, rng);
                }
            }
        }

        self.width = width;
        self.height = height;
    }

    pub fn next_theme(&mut self) {
        self.theme = self.theme.next();
    }

    fn spawn_drops(width: u16, height: u16, rng: &mut SmallRng) -> Vec<RainDrop> {
        let cols = width as usize / COLUMN_STEP + 1;
        let mut drops = Vec::with_capacity(cols * DROPS_PER_COL);

        for col in (0..width).step_by(COLUMN_STEP) {
            Self::push_column(&mut drops, col, height, rng);
        }

        drops
    }

    fn push_column(drops: &mut Vec<RainDrop>, col: u16, height: u16, rng: &mut SmallRng) {
        if rng.random_bool(DENSITY) {
            for _ in 0..DROPS_PER_COL {
                drops.push(RainDrop::new(col, height, rng));
            }
        }
    }
}
