use crate::animations::animation_trait::Animation;
use rand::Rng;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
    Frame,
};

struct Particle {
    x: f32,
    y: f32,
    speed: f32,
}

pub struct Drift {
    width: u16,
    height: u16,
    particles: Vec<Particle>,
    t: f32,
    rng: rand::rngs::ThreadRng,
}

impl Drift {
    pub fn new() -> Self {
        Drift {
            width: 0,
            height: 0,
            particles: Vec::new(),
            t: 0.0,
            rng: rand::thread_rng(),
        }
    }
}

impl Animation for Drift {
    fn init(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.particles.clear();
        let count = 200;
        self.particles.reserve(count);
        for _ in 0..count {
            self.particles.push(Particle {
                x: self.rng.gen_range(0.0..width as f32),
                y: self.rng.gen_range(0.0..height as f32),
                speed: self.rng.gen_range(0.5..2.0),
            });
        }
        self.t = 0.0;
    }

    fn update(&mut self, dt: f32) {
        self.t += dt;
        let w = self.width as f32;
        let h = self.height as f32;
        for p in &mut self.particles {
            let angle = (p.x * 0.05 + self.t).sin() + (p.y * 0.05 - self.t).cos();
            let vx = angle.cos();
            let vy = angle.sin();
            p.x += vx * p.speed * dt * 15.0;
            p.y += vy * p.speed * dt * 15.0;

            if p.x < 0.0 { p.x += w; } else if p.x >= w { p.x -= w; }
            if p.y < 0.0 { p.y += h; } else if p.y >= h { p.y -= h; }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.size();
        let block = Block::default().borders(Borders::NONE);
        block.render(area, frame.buffer_mut());

        let buf = frame.buffer_mut();
        for p in &self.particles {
            let x = p.x.round() as u16;
            let y = p.y.round() as u16;
            if x < self.width && y < self.height {
                let (ch, color) = if p.speed < 0.8 {
                    ('.', Color::Blue)
                } else if p.speed < 1.5 {
                    ('*', Color::Cyan)
                } else {
                    ('o', Color::White)
                };
                buf.set_string(x, y, &ch.to_string(), Style::default().fg(color));
            }
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        for p in &mut self.particles {
            if p.x >= width as f32 { p.x %= width as f32; }
            if p.y >= height as f32 { p.y %= height as f32; }
        }
    }
}