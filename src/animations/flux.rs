use crate::animations::animation_trait::Animation;
use rand::Rng;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
    Frame,
};

#[derive(Clone, Copy)]
enum Dir { NE, SE, SW, NW }

impl Dir {
    fn offsets(&self) -> (i16, i16) {
        match self {
            Dir::NE => (-1, 1), Dir::SE => (1, 1),
            Dir::SW => (1, -1), Dir::NW => (-1, -1),
        }
    }
    fn valid_turns(&self) -> [Dir; 2] {
        match self {
            Dir::NE => [Dir::NW, Dir::SE], Dir::SE => [Dir::NE, Dir::SW],
            Dir::SW => [Dir::SE, Dir::NW], Dir::NW => [Dir::SW, Dir::NE],
        }
    }
}

struct Walker { x: i16, y: i16, dir: Dir, life: u16, age: u16 }

pub struct Flux {
    width: u16, height: u16,
    grid: Vec<Vec<bool>>, age_grid: Vec<Vec<u16>>,
    walkers: Vec<Walker>,
    rng: rand::rngs::ThreadRng,
    filled_count: usize, total_cells: usize,
    tick_accumulator: f32, pause_timer: f32, is_paused: bool,
}

impl Flux {
    pub fn new() -> Self {
        Flux { width: 0, height: 0, grid: Vec::new(), age_grid: Vec::new(), walkers: Vec::new(), rng: rand::thread_rng(), filled_count: 0, total_cells: 0, tick_accumulator: 0.0, pause_timer: 0.0, is_paused: false }
    }
    fn spawn_walker(&mut self) {
        if self.width == 0 || self.height == 0 { return; }
        for _ in 0..20 {
            let x = self.rng.gen_range(0..self.width as i16);
            let y = self.rng.gen_range(0..self.height as i16);
            if !self.grid[y as usize][x as usize] {
                let dir = match self.rng.gen_range(0..4) { 0 => Dir::NE, 1 => Dir::SE, 2 => Dir::SW, _ => Dir::NW };
                self.walkers.push(Walker { x, y, dir, life: self.rng.gen_range(50..=200), age: 0 });
                return;
            }
        }
    }
    fn reset(&mut self) {
        self.grid = vec![vec![false; self.width as usize]; self.height as usize];
        self.age_grid = vec![vec![0; self.width as usize]; self.height as usize];
        self.walkers.clear(); self.filled_count = 0;
        self.total_cells = self.width as usize * self.height as usize;
        self.tick_accumulator = 0.0; self.pause_timer = 0.0; self.is_paused = false;
        for _ in 0..4 { self.spawn_walker(); }
    }
}

impl Animation for Flux {
    fn init(&mut self, width: u16, height: u16) { self.width = width; self.height = height; self.reset(); }
    fn update(&mut self, dt: f32) {
        if self.width == 0 || self.height == 0 { return; }
        if self.is_paused { self.pause_timer -= dt; if self.pause_timer <= 0.0 { self.reset(); } return; }
        self.tick_accumulator += dt;
        if self.tick_accumulator < 0.04 { return; }
        self.tick_accumulator -= 0.04;
        if self.filled_count as f32 / self.total_cells as f32 >= 0.40 { self.is_paused = true; self.pause_timer = 1.5; return; }

        let mut i = 0;
        while i < self.walkers.len() {
            let w = &self.walkers[i];
            let (dx, dy) = w.dir.offsets();
            let (nx, ny) = (w.x + dx, w.y + dy);
            if nx < 0 || nx >= self.width as i16 || ny < 0 || ny >= self.height as i16 || self.grid[ny as usize][nx as usize] {
                self.walkers.remove(i); continue;
            }
            let mut w = self.walkers.remove(i);
            if self.rng.gen_bool(0.3) { let t = w.dir.valid_turns(); w.dir = t[self.rng.gen_range(0..2)]; }
            let (dx, dy) = w.dir.offsets();
            let (nx, ny) = (w.x + dx, w.y + dy);
            if nx < 0 || nx >= self.width as i16 || ny < 0 || ny >= self.height as i16 || self.grid[ny as usize][nx as usize] { continue; }
            self.grid[ny as usize][nx as usize] = true;
            self.age_grid[ny as usize][nx as usize] = 0;
            self.filled_count += 1;
            w.x = nx; w.y = ny; w.age += 1;
            if self.rng.gen_bool(0.1) {
                let t = w.dir.valid_turns();
                self.walkers.push(Walker { x: w.x, y: w.y, dir: t[self.rng.gen_range(0..2)], life: self.rng.gen_range(50..=200), age: 0 });
            }
            w.life = w.life.saturating_sub(1);
            if w.life > 0 { self.walkers.insert(i, w); i += 1; }
        }
        if self.walkers.len() < 3 { self.spawn_walker(); }
        for row in &mut self.age_grid { for a in row { if *a != 0 { *a = a.saturating_add(1); } } }
    }
    fn render(&self, frame: &mut Frame) {
        let area = frame.size();
        Block::default().borders(Borders::NONE).render(area, frame.buffer_mut());
        let buf = frame.buffer_mut();
        let rw = self.width.min(area.width) as usize;
        let rh = self.height.min(area.height) as usize;
        for y in 0..rh { for x in 0..rw {
            if self.grid[y][x] {
                let ch = if (x + y) % 2 == 0 { "/" } else { "\\" };
                let color = if self.age_grid[y][x] < 5 { Color::Cyan } else { Color::Green };
                buf.set_string(x as u16, y as u16, ch, Style::default().fg(color));
            }
        }}
    }
    fn resize(&mut self, width: u16, height: u16) { self.width = width; self.height = height; self.reset(); }
}