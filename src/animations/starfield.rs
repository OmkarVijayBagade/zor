use crate::animations::animation_trait::Animation;
use rand::Rng;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
    Frame,
};

const STAR_CHARS: [char; 3] = ['.', '*', '+'];

#[derive(Clone, Copy)]
struct Star {
    x: u16,
    y: f32,
    speed: f32,
    char_idx: usize,
}

pub struct Starfield {
    stars: Vec<Star>,
    width: u16,
    height: u16,
    rng: rand::rngs::ThreadRng,
}

impl Starfield {
    pub fn new() -> Self {
        Starfield {
            stars: Vec::new(),
            width: 0,
            height: 0,
            rng: rand::thread_rng(),
        }
    }

    fn spawn_star(&mut self) {
        let speed = self.rng.gen_range(0.5..2.0);
        let char_idx = if speed < 0.8 {
            0
        } else if speed < 1.5 {
            1
        } else {
            2
        };
        let star = Star {
            x: self.rng.gen_range(0..self.width),
            y: self.rng.gen_range(0.0..self.height as f32),
            speed,
            char_idx,
        };
        self.stars.push(star);
    }
}

impl Animation for Starfield {
    fn init(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.stars.clear();
        let num_stars = (width * height / 100) as usize;
        for _ in 0..num_stars {
            self.spawn_star();
        }
    }

    fn update(&mut self, dt: f32) {
        for star in &mut self.stars {
            star.y += star.speed * dt * 20.0;
            if star.y >= self.height as f32 {
                star.x = self.rng.gen_range(0..self.width);
                star.y = 0.0;
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.size();
        let block = Block::default().borders(Borders::NONE);
        block.render(area, frame.buffer_mut());

        let buf = frame.buffer_mut();
        for star in &self.stars {
            let x = star.x as u16;
            let y = star.y as u16;
            if x < self.width && y < self.height {
                let ch = STAR_CHARS[star.char_idx];
                let color = match star.char_idx {
                    0 => Color::DarkGray,
                    1 => Color::Gray,
                    2 => Color::White,
                    _ => Color::White,
                };
                buf.set_string(x, y, &ch.to_string(), Style::default().fg(color));
            }
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.stars.clear();
        let num_stars = (width * height / 100) as usize;
        for _ in 0..num_stars {
            self.spawn_star();
        }
    }
}