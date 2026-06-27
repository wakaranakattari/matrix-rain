use rand::{Rng, rngs::SmallRng};

const CHARS: &[char] = &[
    'ア', 'イ', 'ウ', 'エ', 'オ', 'カ', 'キ', 'ク', 'ケ', 'コ', 'サ', 'シ', 'ス', 'セ', 'ソ', 'タ',
    'チ', 'ツ', 'テ', 'ト', '0', '1', '2', '3', '4', '5', '6', '7', '8', '9', 'Z', 'X', 'T', 'R',
    'A', 'N', 'S', 'F', 'O', 'M',
];

pub struct RainDrop {
    pub col: u16,
    pub head: f32,
    pub length: usize,
    pub speed: f32,
    chars: Vec<char>,
}

impl RainDrop {
    pub fn new(col: u16, height: u16, rng: &mut SmallRng) -> Self {
        let length = rng.random_range(8..24);

        let mut chars = Vec::with_capacity(length);

        for _ in 0..length {
            chars.push(random_char(rng));
        }

        Self {
            col,
            head: -(rng.random_range(0.0..height as f32)),
            length,
            speed: rng.random_range(12.0..35.0),
            chars,
        }
    }

    pub fn step(&mut self, dt: f32, height: u16, rng: &mut SmallRng) {
        self.head += self.speed * dt;

        if rng.random_bool(0.18) {
            let idx = rng.random_range(0..self.length);
            self.chars[idx] = random_char(rng);
        }

        if self.head - self.length as f32 > height as f32 {
            self.reset(height, rng);
        }
    }

    pub fn head_row(&self) -> i32 {
        self.head.floor() as i32
    }

    pub fn row_range(&self) -> (i32, i32) {
        let head = self.head_row();
        (head - self.length as i32 + 1, head)
    }

    pub fn cell(&self, row: i32) -> (char, u8) {
        let head = self.head_row();
        let dist = (head - row) as usize;

        let brightness = if dist == 0 {
            3
        } else if dist < self.length / 3 {
            2
        } else if dist < self.length * 2 / 3 {
            1
        } else {
            0
        };

        (self.chars[dist], brightness)
    }

    fn reset(&mut self, height: u16, rng: &mut SmallRng) {
        self.length = rng.random_range(8..24);
        self.speed = rng.random_range(12.0..35.0);
        self.head = -(rng.random_range(0.0..height as f32));

        self.chars.clear();

        for _ in 0..self.length {
            self.chars.push(random_char(rng));
        }
    }
}

fn random_char(rng: &mut SmallRng) -> char {
    CHARS[rng.random_range(0..CHARS.len())]
}
