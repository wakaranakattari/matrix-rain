use rand::{Rng, rngs::SmallRng};

use crate::theme::Brightness;

const CHARS: [char; 40] = [
    'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ', 'ソ', 'タ',
    'チ', 'ツ', 'テ', 'ト', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'Z', 'X', 'T', 'R',
    'A', 'N', 'S', 'F', 'O', 'M',
];

const MIN_LENGTH: usize = 8;
const MAX_LENGTH: usize = 24;
const SPEED_MIN: f32 = 12.0;
const SPEED_MAX: f32 = 35.0;
const MUTATE_PROBABILITY: f64 = 0.18;

pub struct RainDrop {
    col: u16,
    head: f32,
    length: usize,
    speed: f32,
    chars: [char; MAX_LENGTH],
}

impl RainDrop {
    pub fn new(col: u16, height: u16, rng: &mut SmallRng) -> Self {
        let length = rng.random_range(MIN_LENGTH..MAX_LENGTH);
        let mut chars = [CHARS[0]; MAX_LENGTH];
        fill_chars(rng, &mut chars, length);

        Self {
            col,
            head: -(rng.random_range(0.0..height as f32)),
            length,
            speed: rng.random_range(SPEED_MIN..SPEED_MAX),
            chars,
        }
    }

    #[inline]
    pub fn col(&self) -> u16 {
        self.col
    }

    pub fn step(&mut self, dt: f32, height: u16, rng: &mut SmallRng) {
        self.head += self.speed * dt;

        if rng.random_bool(MUTATE_PROBABILITY) {
            let idx = rng.random_range(0..self.length);
            self.chars[idx] = random_char(rng);
        }

        if self.head - self.length as f32 > height as f32 {
            self.reset(height, rng);
        }
    }

    #[inline]
    pub fn head_row(&self) -> i32 {
        self.head.floor() as i32
    }

    #[inline]
    pub fn row_range(&self, head: i32) -> (i32, i32) {
        (head - self.length as i32 + 1, head)
    }

    #[inline]
    pub fn thresholds(&self) -> (usize, usize) {
        (self.length / 3, self.length * 2 / 3)
    }

    #[inline]
    pub fn cell(&self, row: i32, head: i32, mid_end: usize, dim_end: usize) -> (char, Brightness) {
        let dist = (head - row) as usize;
        (
            self.chars[dist],
            Brightness::from_dist(dist, mid_end, dim_end),
        )
    }

    fn reset(&mut self, height: u16, rng: &mut SmallRng) {
        self.length = rng.random_range(MIN_LENGTH..MAX_LENGTH);
        self.speed = rng.random_range(SPEED_MIN..SPEED_MAX);
        self.head = -(rng.random_range(0.0..height as f32));
        fill_chars(rng, &mut self.chars, self.length);
    }
}

#[inline]
fn fill_chars(rng: &mut SmallRng, chars: &mut [char; MAX_LENGTH], len: usize) {
    for slot in chars.iter_mut().take(len) {
        *slot = random_char(rng);
    }
}

#[inline]
fn random_char(rng: &mut SmallRng) -> char {
    CHARS[rng.random_range(0..CHARS.len())]
}
