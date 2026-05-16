use crate::animations::animation_trait::Animation;
use rand::Rng;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
    Frame,
};

const DIR_OFFSETS: [(i16, i16); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];

struct Walker {
    x: i16,
    y: i16,
    dir: usize,
    life: u16,
}

pub struct Void {
    width: u16,
    height: u16,
    grid: Vec<Vec<bool>>,
    age: Vec<Vec<u16>>,
    walkers: Vec<Walker>,
    rng: rand::rngs::ThreadRng,
    empty_count: usize,
    total_cells: usize,
    tick_accumulator: f32,
    pause_timer: f32,
    is_paused: bool,
}

impl Void {
    pub fn new() -> Self {
        Void {
            width: 0,
            height: 0,
            grid: Vec::new(),
            age: Vec::new(),
            walkers: Vec::new(),
            rng: rand::thread_rng(),
            empty_count: 0,
            total_cells: 0,
            tick_accumulator: 0.0,
            pause_timer: 0.0,
            is_paused: false,
        }
    }

    fn spawn_walker(&mut self) {
        if self.width == 0 || self.height == 0 { return; }
        let mut attempts = 0;
        while attempts < 20 {
            let x = self.rng.gen_range(0..self.width as i16);
            let y = self.rng.gen_range(0..self.height as i16);
            if self.grid[y as usize][x as usize] {
                self.walkers.push(Walker {
                    x, y,
                    dir: self.rng.gen_range(0..4),
                    life: self.rng.gen_range(100..=300),
                });
                return;
            }
            attempts += 1;
        }
    }

    fn reset(&mut self) {
        self.grid = vec![vec![true; self.width as usize]; self.height as usize];
        self.age = vec![vec![0; self.width as usize]; self.height as usize];
        self.walkers.clear();
        self.empty_count = 0;
        self.total_cells = self.width as usize * self.height as usize;
        self.tick_accumulator = 0.0;
        self.pause_timer = 0.0;
        self.is_paused = false;
        for _ in 0..4 {
            self.spawn_walker();
        }
    }
}

impl Animation for Void {
    fn init(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.reset();
    }

    fn update(&mut self, dt: f32) {
        if self.width == 0 || self.height == 0 { return; }

        if self.is_paused {
            self.pause_timer -= dt;
            if self.pause_timer <= 0.0 {
                self.reset();
            }
            return;
        }

        self.tick_accumulator += dt;
        if self.tick_accumulator < 0.1 {
            return;
        }
        self.tick_accumulator -= 0.1;

        let empty_ratio = self.empty_count as f32 / self.total_cells as f32;
        if empty_ratio >= 0.9 {
            self.is_paused = true;
            self.pause_timer = 2.5;
            return;
        }

        let mut new_walkers = Vec::new();
        let mut i = 0;
        while i < self.walkers.len() {
            let w = &self.walkers[i];
            
            let mut filled_dirs = Vec::new();
            for (dir_idx, (dx, dy)) in DIR_OFFSETS.iter().enumerate() {
                let nx = w.x + dx;
                let ny = w.y + dy;
                if nx >= 0 && nx < self.width as i16 && ny >= 0 && ny < self.height as i16 {
                    if self.grid[ny as usize][nx as usize] {
                        filled_dirs.push(dir_idx);
                    }
                }
            }

            let next_dir = if filled_dirs.is_empty() {
                None
            } else {
                let chosen = if self.rng.gen_bool(0.7) {
                    filled_dirs[self.rng.gen_range(0..filled_dirs.len())]
                } else {
                    self.rng.gen_range(0..4)
                };
                Some(chosen)
            };

            if next_dir.is_none() {
                self.walkers.remove(i);
                continue;
            }

            let dir = next_dir.unwrap();
            let dx = DIR_OFFSETS[dir].0;
            let dy = DIR_OFFSETS[dir].1;
            let nx = w.x + dx;
            let ny = w.y + dy;

            if nx < 0 || nx >= self.width as i16 || ny < 0 || ny >= self.height as i16 {
                self.walkers.remove(i);
                continue;
            }

            let mut w = self.walkers.remove(i);
            w.dir = dir;

            if self.grid[ny as usize][nx as usize] {
                self.grid[ny as usize][nx as usize] = false;
                self.age[ny as usize][nx as usize] = 0;
                self.empty_count += 1;
            }

            w.x = nx;
            w.y = ny;

            if self.rng.gen_bool(0.15) {
                new_walkers.push(Walker {
                    x: w.x, y: w.y,
                    dir: self.rng.gen_range(0..4),
                    life: self.rng.gen_range(100..=300),
                });
            }

            w.life = w.life.saturating_sub(1);
            if w.life > 0 {
                self.walkers.insert(i, w);
                i += 1;
            }
        }

        self.walkers.extend(new_walkers);

        if self.walkers.len() < 4 {
            self.spawn_walker();
        }

        for y in 0..self.height as usize {
            for x in 0..self.width as usize {
                if !self.grid[y][x] {
                    self.age[y][x] = self.age[y][x].saturating_add(1);
                }
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
                if self.grid[y][x] {
                    buf.set_string(x as u16, y as u16, "█", Style::default().fg(Color::DarkGray));
                } else {
                    let a = self.age[y][x];
                    let (ch, color) = if a < 2 {
                        ('@', Color::Cyan)
                    } else if a < 10 {
                        ('*', Color::Blue)
                    } else if a < 30 {
                        ('.', Color::Blue)
                    } else {
                        (' ', Color::Black)
                    };
                    buf.set_string(x as u16, y as u16, &ch.to_string(), Style::default().fg(color));
                }
            }
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.reset();
    }
}