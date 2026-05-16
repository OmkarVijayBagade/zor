use crate::animations::animation_trait::Animation;
use rand::Rng;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
    Frame,
};

struct Drop {
    x: f32,
    y: f32,
    speed: f32,
    length: u16,
    ch: char,
}

struct Splash {
    x: u16,
    y: u16,
    life: u8,
}

pub struct Rain {
    width: u16,
    height: u16,
    drops: Vec<Drop>,
    splashes: Vec<Splash>,
    rng: rand::rngs::ThreadRng,
    wind: f32,
}

impl Rain {
    pub fn new() -> Self {
        Rain {
            width: 0,
            height: 0,
            drops: Vec::new(),
            splashes: Vec::new(),
            rng: rand::thread_rng(),
            wind: 0.3,
        }
    }

    fn spawn_drop(rng: &mut impl Rng, width: u16, _height: u16, above_screen: bool) -> Drop {
        let speed = rng.gen_range(0.5..2.0);
        let length = rng.gen_range(2..=5);
        let x = rng.gen_range(0.0..width as f32);
        let y = if above_screen {
            rng.gen_range(-20.0..0.0)
        } else {
            -(length as f32)
        };
        let ch = match rng.gen_range(0..3) {
            0 => '|',
            1 => '/',
            _ => '\\',
        };
        Drop { x, y, speed, length, ch }
    }
}

impl Animation for Rain {
    fn init(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.drops.clear();
        self.splashes.clear();
        let num_drops = (width * height / 30) as usize;
        self.drops.reserve(num_drops);
        for _ in 0..num_drops {
            self.drops.push(Self::spawn_drop(&mut self.rng, width, height, true));
        }
    }

    fn update(&mut self, dt: f32) {
        let w = self.width;
        let h = self.height;
        for drop in &mut self.drops {
            drop.x += self.wind * dt * 10.0;
            drop.y += drop.speed * dt * 15.0;

            if drop.y > h as f32 {
                if drop.x >= 0.0 && drop.x < w as f32 {
                    self.splashes.push(Splash {
                        x: drop.x.max(0.0) as u16,
                        y: h.saturating_sub(1),
                        life: 2,
                    });
                }
                *drop = Self::spawn_drop(&mut self.rng, w, h, false);
            }
        }

        let mut i = 0;
        while i < self.splashes.len() {
            self.splashes[i].life = self.splashes[i].life.saturating_sub(1);
            if self.splashes[i].life == 0 {
                self.splashes.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.size();
        let block = Block::default().borders(Borders::NONE);
        block.render(area, frame.buffer_mut());

        let buf = frame.buffer_mut();
        let rain_style = Style::default().fg(Color::Cyan);
        let splash_style = Style::default().fg(Color::White);

        for drop in &self.drops {
            let x = drop.x.max(0.0) as u16;
            let y_base = drop.y as i32;
            if x < self.width {
                for i in 0..drop.length {
                    let y = y_base - i as i32;
                    if y >= 0 && y < self.height as i32 {
                        buf.set_string(x, y as u16, &drop.ch.to_string(), rain_style);
                    }
                }
            }
        }

        for splash in &self.splashes {
            if splash.x < self.width && splash.y < self.height {
                buf.set_string(splash.x, splash.y, "*", splash_style);
            }
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.init(width, height);
    }
}