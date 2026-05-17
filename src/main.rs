use std::io;
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode},
    execute,
    terminal::{self, EnterAlternateScreen, LeaveAlternateScreen},
};
use ratatui::{
    backend::CrosstermBackend,
    Terminal,
};

mod app;
mod ui;
mod animations;

use crate::app::App;
use crate::app::InputMode;

fn main() -> Result<(), io::Error> {
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    execute!(stdout, EnterAlternateScreen, EnableMouseCapture)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();
    let tick_rate = Duration::from_millis(33);
    let mut last_tick = Instant::now();
    let mut running = true;

    while running {
        terminal.draw(|f| ui::draw(f, &app))?;

        let timeout = tick_rate
            .checked_sub(last_tick.elapsed())
            .unwrap_or_else(|| Duration::from_secs(0));

        if event::poll(timeout)? {
            if let Event::Key(key) = event::read()? {
                match &mut app.mode {
                    crate::app::AppMode::Menu(state) => {
                        if state.input_mode == InputMode::Normal && key.code == KeyCode::Char('q') {
                            running = false;
                        } else {
                            if let Some(idx) = app.handle_key(key.code) {
                                app.launch_animation(idx);
                            }
                        }
                    }
                    crate::app::AppMode::Running(ref mut anim) => {
                        if !anim.handle_input(Event::Key(key)) {
                            app.return_to_menu();
                        }
                    }
                }
            }
        }

        if last_tick.elapsed() >= tick_rate {
            app.update(tick_rate.as_secs_f32());
            last_tick = Instant::now();
        }
    }

    disable_raw_mode()?;
    execute!(
        terminal.backend_mut(),
        LeaveAlternateScreen,
        DisableMouseCapture
    )?;
    terminal.show_cursor()?;

    Ok(())
}

fn enable_raw_mode() -> Result<(), io::Error> {
    terminal::enable_raw_mode()
}

fn disable_raw_mode() -> Result<(), io::Error> {
    terminal::disable_raw_mode()
}