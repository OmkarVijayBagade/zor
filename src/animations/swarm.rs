use crate::animations::animation_trait::Animation;
use rand::Rng;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
    Frame,
};
use std::f32::consts::PI;

struct Particle {
    angle: f32,
    radius: f32,
    speed: f32,
    direction: f32,
}

pub struct Swarm {
    width: u16,
    height: u16,
    particles: Vec<Particle>,
    t: f32,
    rng: rand::rngs::ThreadRng,
}

impl Swarm {
    pub fn new() -> Self {
        Swarm {
            width: 0,
            height: 0,
            particles: Vec::new(),
            t: 0.0,
            rng: rand::thread_rng(),
        }
    }
}

impl Animation for Swarm {
    fn init(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.particles.clear();
        let count = 150;
        let max_radius = (width.min(height) as f32) / 2.0;
        for _ in 0..count {
            self.particles.push(Particle {
                angle: self.rng.gen_range(0.0..2.0 * PI),
                radius: self.rng.gen_range(5.0..max_radius),
                speed: self.rng.gen_range(0.5..2.0),
                direction: if self.rng.gen_bool(0.5) { 1.0 } else { -1.0 },
            });
        }
        self.t = 0.0;
    }

    fn update(&mut self, dt: f32) {
        self.t += dt;
        let max_radius = (self.width.min(self.height) as f32) / 2.0;
        for p in &mut self.particles {
            p.angle += p.speed * p.direction * dt * 2.0;
            p.radius += (self.t + p.angle).sin() * 0.1;
            p.radius = p.radius.max(2.0).min(max_radius);
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.size();
        let block = Block::default().borders(Borders::NONE);
        block.render(area, frame.buffer_mut());

        let buf = frame.buffer_mut();
        let cx = self.width as f32 / 2.0 + (self.t * 0.5).sin() * (self.width as f32 * 0.2);
        let cy = self.height as f32 / 2.0 + (self.t * 0.3).cos() * (self.height as f32 * 0.1);
        let max_radius = (self.width.min(self.height) as f32) / 2.0;

        for p in &self.particles {
            let x = cx + p.angle.cos() * p.radius;
            let y = cy + p.angle.sin() * p.radius;
            let px = x.round() as u16;
            let py = y.round() as u16;

            if px < self.width && py < self.height {
                let normalized_r = p.radius / max_radius;
                let (ch, color) = if normalized_r < 0.33 {
                    ('@', Color::White)
                } else if normalized_r < 0.66 {
                    ('*', Color::Cyan)
                } else {
                    ('.', Color::Blue)
                };
                buf.set_string(px, py, &ch.to_string(), Style::default().fg(color));
            }
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        let max_radius = (width.min(height) as f32) / 2.0;
        for p in &mut self.particles {
            p.radius = p.radius.max(2.0).min(max_radius);
        }
    }
}