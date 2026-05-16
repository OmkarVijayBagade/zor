use crate::animations::animation_trait::Animation;
use rand::Rng;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
    Frame,
};

const DIR_OFFSETS: [(i16, i16); 4] = [(0, -1), (0, 1), (-1, 0), (1, 0)];
const DIR_MASKS: [u8; 4] = [1, 2, 4, 8];
const OPP_MASKS: [u8; 4] = [2, 1, 8, 4];

struct Walker {
    x: i16,
    y: i16,
    dir: usize,
    life: u16,
    age: u8,
}

pub struct Circuit {
    width: u16,
    height: u16,
    grid: Vec<Vec<u8>>,
    age_grid: Vec<Vec<u8>>,
    walkers: Vec<Walker>,
    rng: rand::rngs::ThreadRng,
    filled_count: usize,
    total_cells: usize,
    tick_accumulator: f32,
    pause_timer: f32,
    is_paused: bool,
}

impl Circuit {
    pub fn new() -> Self {
        Circuit {
            width: 0,
            height: 0,
            grid: Vec::new(),
            age_grid: Vec::new(),
            walkers: Vec::new(),
            rng: rand::thread_rng(),
            filled_count: 0,
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
            if self.grid[y as usize][x as usize] == 0 {
                self.walkers.push(Walker {
                    x, y,
                    dir: self.rng.gen_range(0..4),
                    life: self.rng.gen_range(100..=400),
                    age: 0,
                });
                return;
            }
            attempts += 1;
        }
    }

    fn count_empty_neighbors(&self, x: i16, y: i16) -> usize {
        let mut count = 0;
        for (dx, dy) in &DIR_OFFSETS {
            let nx = x + dx;
            let ny = y + dy;
            if nx >= 0 && nx < self.width as i16 && ny >= 0 && ny < self.height as i16 {
                if self.grid[ny as usize][nx as usize] == 0 {
                    count += 1;
                }
            }
        }
        count
    }

    fn reset(&mut self) {
        self.grid = vec![vec![0; self.width as usize]; self.height as usize];
        self.age_grid = vec![vec![0; self.width as usize]; self.height as usize];
        self.walkers.clear();
        self.filled_count = 0;
        self.total_cells = self.width as usize * self.height as usize;
        self.tick_accumulator = 0.0;
        self.pause_timer = 0.0;
        self.is_paused = false;
        for _ in 0..5 {
            self.spawn_walker();
        }
    }
}

impl Animation for Circuit {
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
        if self.tick_accumulator < 0.15 {
            return;
        }
        self.tick_accumulator -= 0.15;

        let filled_ratio = self.filled_count as f32 / self.total_cells as f32;
        if filled_ratio >= 0.9 {
            self.is_paused = true;
            self.pause_timer = 2.5;
            return;
        }

        let mut new_walkers = Vec::new();
        let mut i = 0;
        while i < self.walkers.len() {
            let w = &self.walkers[i];
            
            let mut valid_dirs = Vec::new();
            for (dir_idx, (dx, dy)) in DIR_OFFSETS.iter().enumerate() {
                let nx = w.x + dx;
                let ny = w.y + dy;
                if nx >= 0 && nx < self.width as i16 && ny >= 0 && ny < self.height as i16 {
                    if self.grid[ny as usize][nx as usize] == 0 {
                        valid_dirs.push(dir_idx);
                    }
                }
            }

            let next_dir = if valid_dirs.is_empty() {
                None
            } else {
                let chosen = valid_dirs[self.rng.gen_range(0..valid_dirs.len())];
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

            if self.grid[ny as usize][nx as usize] != 0 {
                self.walkers.remove(i);
                continue;
            }

            let mut w = self.walkers.remove(i);
            w.dir = dir;
            w.age = w.age.saturating_add(1);

            if self.grid[w.y as usize][w.x as usize] == 0 {
                self.filled_count += 1;
            }
            self.grid[w.y as usize][w.x as usize] |= DIR_MASKS[w.dir];
            self.age_grid[w.y as usize][w.x as usize] = w.age;

            if self.grid[ny as usize][nx as usize] == 0 {
                self.filled_count += 1;
            }
            self.grid[ny as usize][nx as usize] |= OPP_MASKS[w.dir];
            self.age_grid[ny as usize][nx as usize] = w.age;

            w.x = nx;
            w.y = ny;

            if self.count_empty_neighbors(w.x, w.y) >= 2 && self.rng.gen_bool(0.12) {
                new_walkers.push(Walker {
                    x: w.x, y: w.y,
                    dir: self.rng.gen_range(0..4),
                    life: self.rng.gen_range(100..=400),
                    age: 0,
                });
            }

            w.life = w.life.saturating_sub(1);
            if w.life > 0 {
                self.walkers.insert(i, w);
                i += 1;
            }
        }

        self.walkers.extend(new_walkers);

        if self.walkers.len() < 5 {
            self.spawn_walker();
        }
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.size();
        let block = Block::default().borders(Borders::NONE);
        block.render(area, frame.buffer_mut());

        let buf = frame.buffer_mut();
        let render_w = self.width.min(area.width) as usize;
        let render_h = self.height.min(area.height) as usize;

        const CHARS: [&str; 16] = [
            " ", "│", "│", "│",
            "─", "└", "┌", "├",
            "─", "┘", "┐", "┤",
            "─", "┴", "┬", "┼",
        ];

        for y in 0..render_h {
            for x in 0..render_w {
                let mask = self.grid[y][x];
                if mask != 0 {
                    let age = self.age_grid[y][x];
                    let color = if mask == 15 {
                        Color::White
                    } else if age < 10 {
                        Color::LightGreen
                    } else {
                        Color::Green
                    };
                    buf.set_string(x as u16, y as u16, CHARS[mask as usize], Style::default().fg(color));
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