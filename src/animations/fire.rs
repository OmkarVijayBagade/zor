use crate::animations::animation_trait::Animation;
use rand::Rng;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
    Frame,
};

pub struct Fire {
    width: u16,
    height: u16,
    heat: Vec<Vec<u8>>,
    rng: rand::rngs::ThreadRng,
}

impl Fire {
    pub fn new() -> Self {
        Fire {
            width: 0,
            height: 0,
            heat: Vec::new(),
            rng: rand::thread_rng(),
        }
    }
}

impl Animation for Fire {
    fn init(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.heat = vec![vec![0; width as usize]; height as usize];
    }

    fn update(&mut self, _dt: f32) {
        let w = self.width as usize;
        let h = self.height as usize;
        if w == 0 || h == 0 {
            return;
        }

        for x in 0..w {
            self.heat[h - 1][x] = self.rng.gen_range(180..=255);
        }

        for y in (1..h).rev() {
            for x in 0..w {
                let below = self.heat[y][x] as u16;
                let left = if x > 0 {
                    self.heat[y][x - 1] as u16
                } else {
                    0
                };
                let right = if x < w - 1 {
                    self.heat[y][x + 1] as u16
                } else {
                    0
                };

                let avg = (below + left + right) / 3;
                let cooling = self.rng.gen_range(0..=3) as u16;
                self.heat[y - 1][x] = avg.saturating_sub(cooling).min(255) as u8;
            }
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.size();
        let block = Block::default().borders(Borders::NONE);
        block.render(area, frame.buffer_mut());

        let buf = frame.buffer_mut();
        let render_w = self.width.min(area.width) as usize;
        let render_h = self.height.min(area.height) as usize;

        for y in 0..render_h {
            for x in 0..render_w {
                let val = self.heat[y][x];
                let (ch, color) = match val {
                    0..=50 => (' ', Color::Black),
                    51..=100 => ('.', Color::Red),
                    101..=150 => ('*', Color::Yellow),
                    151..=200 => ('o', Color::LightYellow),
                    _ => ('@', Color::White),
                };
                buf.set_string(x as u16, y as u16, &ch.to_string(), Style::default().fg(color));
            }
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.init(width, height);
    }
}