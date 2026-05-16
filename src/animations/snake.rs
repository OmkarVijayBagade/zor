use crate::animations::animation_trait::Animation;
use rand::Rng;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
    Frame,
};

pub struct Snake {
    width: u16,
    height: u16,
    snake: Vec<(u16, u16)>,
    direction: (i16, i16),
    food: (u16, u16),
    rng: rand::rngs::ThreadRng,
    tick_accumulator: f32,
}

impl Snake {
    pub fn new() -> Self {
        Snake {
            width: 0,
            height: 0,
            snake: Vec::new(),
            direction: (1, 0),
            food: (0, 0),
            rng: rand::thread_rng(),
            tick_accumulator: 0.0,
        }
    }

    fn reset(&mut self) {
        let cx = self.width / 2;
        let cy = self.height / 2;
        let len = self.rng.gen_range(3..=5);
        self.snake.clear();
        for i in 0..len {
            self.snake.push((cx.saturating_sub(i as u16), cy));
        }
        self.direction = (1, 0);
        self.spawn_food();
    }

    fn spawn_food(&mut self) {
        if self.width == 0 || self.height == 0 {
            return;
        }
        loop {
            let x = self.rng.gen_range(0..self.width);
            let y = self.rng.gen_range(0..self.height);
            if !self.snake.contains(&(x, y)) {
                self.food = (x, y);
                break;
            }
        }
    }

    fn is_safe(&self, x: i16, y: i16) -> bool {
        if x < 0 || x >= self.width as i16 || y < 0 || y >= self.height as i16 {
            return false;
        }
        let pos = (x as u16, y as u16);
        if self.snake.len() > 1 && self.snake.last() == Some(&pos) {
            return true;
        }
        !self.snake.contains(&pos)
    }

    fn update_direction(&mut self) {
        let head = self.snake[0];
        let fx = self.food.0 as i16;
        let fy = self.food.1 as i16;
        let hx = head.0 as i16;
        let hy = head.1 as i16;

        let mut candidates = Vec::new();

        if fx > hx {
            candidates.push((1, 0));
        } else if fx < hx {
            candidates.push((-1, 0));
        }

        if fy > hy {
            candidates.push((0, 1));
        } else if fy < hy {
            candidates.push((0, -1));
        }

        candidates.push(self.direction);

        for dir in candidates {
            let nx = hx + dir.0;
            let ny = hy + dir.1;
            if self.is_safe(nx, ny) {
                self.direction = dir;
                return;
            }
        }

        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
        for dir in dirs {
            let nx = hx + dir.0;
            let ny = hy + dir.1;
            if self.is_safe(nx, ny) {
                self.direction = dir;
                return;
            }
        }
    }

    fn tick(&mut self) {
        self.update_direction();

        let head = self.snake[0];
        let hx = head.0 as i16 + self.direction.0;
        let hy = head.1 as i16 + self.direction.1;

        if hx < 0 || hx >= self.width as i16 || hy < 0 || hy >= self.height as i16 {
            self.reset();
            return;
        }

        let new_head = (hx as u16, hy as u16);

        if self.snake.contains(&new_head) {
            self.reset();
            return;
        }

        self.snake.insert(0, new_head);

        if new_head == self.food {
            self.spawn_food();
        } else {
            self.snake.pop();
        }
    }
}

impl Animation for Snake {
    fn init(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.tick_accumulator = 0.0;
        self.reset();
    }

    fn update(&mut self, dt: f32) {
        self.tick_accumulator += dt;
        if self.tick_accumulator >= 0.1 {
            self.tick_accumulator -= 0.1;
            self.tick();
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.size();
        let block = Block::default().borders(Borders::NONE);
        block.render(area, frame.buffer_mut());

        let buf = frame.buffer_mut();

        buf.set_string(
            self.food.0,
            self.food.1,
            "*",
            Style::default().fg(Color::Red),
        );

        for (i, &(x, y)) in self.snake.iter().enumerate() {
            let ch = if i == 0 { "@" } else { "o" };
            let color = if i == 0 { Color::LightGreen } else { Color::Green };
            buf.set_string(x, y, ch, Style::default().fg(color));
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        let out_of_bounds = self.snake.iter().any(|&(x, y)| x >= width || y >= height);
        if out_of_bounds {
            self.reset();
        }
    }
}