use crate::animations::starfield::Starfield;
use crate::animations::animation_trait::Animation;

const MENU_ITEMS: usize = 4;

pub enum AppMode {
    Menu { selected: usize },
    Running(Box<dyn Animation>),
}

pub struct App {
    pub mode: AppMode,
}

impl App {
    pub fn new() -> Self {
        App {
            mode: AppMode::Menu { selected: 0 },
        }
    }

    pub fn update(&mut self, dt: f32) {
        if let AppMode::Running(ref mut anim) = self.mode {
            anim.update(dt);
        }
    }

    pub fn resize(&mut self, width: u16, height: u16) {
        if let AppMode::Running(ref mut anim) = self.mode {
            anim.resize(width, height);
        }
    }

    pub fn select_next(&mut self) {
        if let AppMode::Menu { selected } = &mut self.mode {
            *selected = (*selected + 1).min(MENU_ITEMS - 1);
        }
    }

    pub fn select_previous(&mut self) {
        if let AppMode::Menu { selected } = &mut self.mode {
            *selected = selected.saturating_sub(1);
        }
    }

    pub fn select_index(&mut self, index: usize) {
        if let AppMode::Menu { selected } = &mut self.mode {
            *selected = index.min(MENU_ITEMS - 1);
        }
    }

    pub fn launch_selected(&mut self) {
        let selected = match &self.mode {
            AppMode::Menu { selected } => *selected,
            _ => return,
        };

        let mut new_anim: Box<dyn Animation> = match selected {
            0 => Box::new(Starfield::new()),
            1 => Box::new(crate::animations::matrix::Matrix::new()),
            2 => Box::new(crate::animations::wave::Wave::new()),
            3 => Box::new(crate::animations::snake::Snake::new()),
            _ => Box::new(Starfield::new()),
        };

        let size = crossterm::terminal::size().unwrap_or((80, 24));
        new_anim.init(size.0, size.1);
        self.mode = AppMode::Running(new_anim);
    }
}