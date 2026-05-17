use crate::animations::starfield::Starfield;
use crate::animations::animation_trait::Animation;

#[derive(PartialEq)]
pub enum InputMode {
    Normal,
    Search,
    Number,
}

pub struct MenuState {
    pub selected_index: usize,
    pub input_mode: InputMode,
    pub items: Vec<String>,
    pub filtered_indices: Vec<usize>,
    pub search_query: String,
    pub number_buffer: String,
}

impl MenuState {
    fn new() -> Self {
        let items = vec![
            "Starfield".to_string(),
            "Matrix".to_string(),
            "Wave".to_string(),
            "Snake".to_string(),
            "Fire".to_string(),
            "Rain".to_string(),
            "Swarm".to_string(),
            "Circuit".to_string(),
            "Void".to_string(),
            "Flux".to_string(),
        ];
        MenuState {
            selected_index: 0,
            input_mode: InputMode::Normal,
            filtered_indices: (0..items.len()).collect(),
            search_query: String::new(),
            number_buffer: String::new(),
            items,
        }
    }
}

pub enum AppMode {
    Menu(MenuState),
    Running(Box<dyn Animation>),
}

pub struct App {
    pub mode: AppMode,
}

impl App {
    pub fn new() -> Self {
        App {
            mode: AppMode::Menu(MenuState::new()),
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

    fn update_filter(&mut self) {
        if let AppMode::Menu(state) = &mut self.mode {
            let query = state.search_query.to_lowercase();
            state.filtered_indices = state.items.iter().enumerate()
                .filter(|(_, item)| item.to_lowercase().contains(&query))
                .map(|(i, _)| i)
                .collect();
            if !state.filtered_indices.is_empty() {
                state.selected_index = 0;
            }
        }
    }

    pub fn handle_key(&mut self, key: crossterm::event::KeyCode) {
        match &mut self.mode {
            AppMode::Menu(state) => {
                match state.input_mode {
                    InputMode::Normal => {
                        match key {
                            crossterm::event::KeyCode::Up => {
                                if !state.filtered_indices.is_empty() {
                                    state.selected_index = state.selected_index.saturating_sub(1);
                                }
                            }
                            crossterm::event::KeyCode::Down => {
                                if !state.filtered_indices.is_empty() {
                                    state.selected_index = (state.selected_index + 1).min(state.filtered_indices.len().saturating_sub(1));
                                }
                            }
                            crossterm::event::KeyCode::Char('k') => {
                                if !state.filtered_indices.is_empty() {
                                    state.selected_index = state.selected_index.saturating_sub(1);
                                }
                            }
                            crossterm::event::KeyCode::Char('j') => {
                                if !state.filtered_indices.is_empty() {
                                    state.selected_index = (state.selected_index + 1).min(state.filtered_indices.len().saturating_sub(1));
                                }
                            }
                            crossterm::event::KeyCode::Enter => {
                                self.launch_selected();
                            }
                            crossterm::event::KeyCode::Char('/') => {
                                state.input_mode = InputMode::Search;
                                state.search_query.clear();
                                state.filtered_indices = (0..state.items.len()).collect();
                                state.selected_index = 0;
                            }
                            crossterm::event::KeyCode::Char(c) if c.is_ascii_digit() => {
                                state.input_mode = InputMode::Number;
                                state.number_buffer.clear();
                                state.number_buffer.push(c);
                            }
                            _ => {}
                        }
                    }
                    InputMode::Number => {
                        match key {
                            crossterm::event::KeyCode::Char(c) if c.is_ascii_digit() => {
                                if state.number_buffer.len() < 2 {
                                    state.number_buffer.push(c);
                                }
                            }
                            crossterm::event::KeyCode::Enter => {
                                let target = state.number_buffer.parse::<usize>().ok().and_then(|num| {
                                    if num >= 1 && num <= state.items.len() {
                                        let idx = num - 1;
                                        state.filtered_indices.iter().position(|&i| i == idx).map(|pos| (pos, idx))
                                    } else {
                                        None
                                    }
                                });
                                
                                if let Some((pos, anim_idx)) = target {
                                    state.selected_index = pos;
                                    let mut new_anim: Box<dyn Animation> = match anim_idx {
                                        0 => Box::new(Starfield::new()),
                                        1 => Box::new(crate::animations::matrix::Matrix::new()),
                                        2 => Box::new(crate::animations::wave::Wave::new()),
                                        3 => Box::new(crate::animations::snake::Snake::new()),
                                        4 => Box::new(crate::animations::fire::Fire::new()),
                                        5 => Box::new(crate::animations::rain::Rain::new()),
                                        6 => Box::new(crate::animations::swarm::Swarm::new()),
                                        7 => Box::new(crate::animations::circuit::Circuit::new()),
                                        8 => Box::new(crate::animations::void::Void::new()),
                                        9 => Box::new(crate::animations::flux::Flux::new()),
                                        _ => Box::new(Starfield::new()),
                                    };
                                    let size = crossterm::terminal::size().unwrap_or((80, 24));
                                    new_anim.init(size.0, size.1);
                                    self.mode = AppMode::Running(new_anim);
                                } else {
                                    state.number_buffer.clear();
                                    state.input_mode = InputMode::Normal;
                                }
                            }
                            crossterm::event::KeyCode::Esc | crossterm::event::KeyCode::Backspace => {
                                state.number_buffer.clear();
                                state.input_mode = InputMode::Normal;
                            }
                            _ => {}
                        }
                    }
                    InputMode::Search => {
                        match key {
                            crossterm::event::KeyCode::Char('k') => {
                                if !state.filtered_indices.is_empty() {
                                    state.selected_index = state.selected_index.saturating_sub(1);
                                }
                            }
                            crossterm::event::KeyCode::Char('j') => {
                                if !state.filtered_indices.is_empty() {
                                    state.selected_index = (state.selected_index + 1).min(state.filtered_indices.len().saturating_sub(1));
                                }
                            }
                            crossterm::event::KeyCode::Char(c) => {
                                state.search_query.push(c);
                                self.update_filter();
                            }
                            crossterm::event::KeyCode::Backspace => {
                                state.search_query.pop();
                                self.update_filter();
                            }
                            crossterm::event::KeyCode::Up => {
                                if !state.filtered_indices.is_empty() {
                                    state.selected_index = state.selected_index.saturating_sub(1);
                                }
                            }
                            crossterm::event::KeyCode::Down => {
                                if !state.filtered_indices.is_empty() {
                                    state.selected_index = (state.selected_index + 1).min(state.filtered_indices.len().saturating_sub(1));
                                }
                            }
                            crossterm::event::KeyCode::Enter => {
                                self.launch_selected();
                            }
                            crossterm::event::KeyCode::Esc => {
                                state.search_query.clear();
                                state.filtered_indices = (0..state.items.len()).collect();
                                state.selected_index = 0;
                                state.input_mode = InputMode::Normal;
                            }
                            _ => {}
                        }
                    }
                }
            }
            AppMode::Running(_) => {}
        }
    }

    pub fn launch_selected(&mut self) {
        let target_idx = match &self.mode {
            AppMode::Menu(state) => {
                if state.filtered_indices.is_empty() { return; }
                state.filtered_indices[state.selected_index]
            }
            _ => return,
        };

        let mut new_anim: Box<dyn Animation> = match target_idx {
            0 => Box::new(Starfield::new()),
            1 => Box::new(crate::animations::matrix::Matrix::new()),
            2 => Box::new(crate::animations::wave::Wave::new()),
            3 => Box::new(crate::animations::snake::Snake::new()),
            4 => Box::new(crate::animations::fire::Fire::new()),
            5 => Box::new(crate::animations::rain::Rain::new()),
            6 => Box::new(crate::animations::swarm::Swarm::new()),
            7 => Box::new(crate::animations::circuit::Circuit::new()),
            8 => Box::new(crate::animations::void::Void::new()),
            9 => Box::new(crate::animations::flux::Flux::new()),
            _ => Box::new(Starfield::new()),
        };

        let size = crossterm::terminal::size().unwrap_or((80, 24));
        new_anim.init(size.0, size.1);
        self.mode = AppMode::Running(new_anim);
    }

    pub fn return_to_menu(&mut self) {
        self.mode = AppMode::Menu(MenuState::new());
    }
}