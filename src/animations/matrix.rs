use crate::animations::animation_trait::Animation;
use rand::Rng;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
    Frame,
};

const CHARSET: &[char] = &[
    'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M',
    'N', 'O', 'P', 'Q', 'R', 'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    '0', '1', '2', '3', '4', '5', '6', '7', '8', '9',
    '!', '@', '#', '$', '%', '^', '&', '*', '(', ')', '-', '_', '+', '=',
];

struct Column {
    head_y: f32,
    speed: f32,
    length: u16,
    chars: Vec<char>,
}

impl Column {
    fn new(rng: &mut impl Rng, _width: u16, _height: u16) -> Self {
        let length = rng.gen_range(5..=20);
        let chars = (0..length)
            .map(|_| CHARSET[rng.gen_range(0..CHARSET.len())])
            .collect();
        Column {
            head_y: rng.gen_range(-(length as f32)..0.0),
            speed: rng.gen_range(0.5..2.0),
            length,
            chars,
        }
    }

    fn reset(&mut self, rng: &mut impl Rng, _height: u16) {
        self.length = rng.gen_range(5..=20);
        self.chars.resize_with(self.length as usize, || {
            CHARSET[rng.gen_range(0..CHARSET.len())]
        });
        self.head_y = rng.gen_range(-(self.length as f32)..0.0);
        self.speed = rng.gen_range(0.5..2.0);
    }

    fn mutate_chars(&mut self, rng: &mut impl Rng) {
        for ch in &mut self.chars {
            if rng.gen_bool(0.05) {
                *ch = CHARSET[rng.gen_range(0..CHARSET.len())];
            }
        }
    }
}

pub struct Matrix {
    width: u16,
    height: u16,
    columns: Vec<Column>,
    rng: rand::rngs::ThreadRng,
}

impl Matrix {
    pub fn new() -> Self {
        Matrix {
            width: 0,
            height: 0,
            columns: Vec::new(),
            rng: rand::thread_rng(),
        }
    }
}

impl Animation for Matrix {
    fn init(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.columns.clear();
        self.columns.reserve(width as usize);
        for _ in 0..width {
            self.columns.push(Column::new(&mut self.rng, width, height));
        }
    }

    fn update(&mut self, dt: f32) {
        for col in &mut self.columns {
            col.head_y += col.speed * dt * 15.0;
            col.mutate_chars(&mut self.rng);

            if col.head_y - col.length as f32 > self.height as f32 {
                col.reset(&mut self.rng, self.height);
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.size();
        let block = Block::default().borders(Borders::NONE);
        block.render(area, frame.buffer_mut());

        let buf = frame.buffer_mut();
        let render_width = self.width.min(area.width);

        for x in 0..render_width {
            let col = &self.columns[x as usize];
            let len = col.length.min(self.height);
            let head_y_int = col.head_y.round() as i32;

            for i in 0..len {
                let y = head_y_int - (i as i32);
                if y >= 0 && y < self.height as i32 {
                    let intensity = 1.0 - (i as f32 / len as f32);
                    let ch = col.chars[i as usize];

                    let color = if i == 0 {
                        Color::White
                    } else if intensity > 0.7 {
                        Color::LightGreen
                    } else if intensity > 0.3 {
                        Color::Green
                    } else {
                        Color::DarkGray
                    };

                    buf.set_string(x as u16, y as u16, &ch.to_string(), Style::default().fg(color));
                }
            }
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        let current_len = self.columns.len();
        if width as usize > current_len {
            for _ in 0..(width as usize - current_len) {
                self.columns.push(Column::new(&mut self.rng, width, height));
            }
        } else if (width as usize) < current_len {
            self.columns.truncate(width as usize);
        }
    }
}