use std::io;
use std::time::{Duration, Instant};
use crossterm::{
    event::{self, DisableMouseCapture, EnableMouseCapture, Event, KeyCode, MouseEventKind},
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

const MENU_LIST_START_ROW: usize = 7;

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
            match event::read()? {
                Event::Key(key) => {
                    match &mut app.mode {
                        crate::app::AppMode::Menu { .. } => {
                            match key.code {
                                KeyCode::Char('q') => running = false,
                                KeyCode::Up | KeyCode::Char('k') => app.select_previous(),
                                KeyCode::Down | KeyCode::Char('j') => app.select_next(),
                                KeyCode::Enter => app.launch_selected(),
                                _ => {}
                            }
                        }
                        crate::app::AppMode::Running(ref mut anim) => {
                            if !anim.handle_input(Event::Key(key)) {
                                app.mode = crate::app::AppMode::Menu { selected: 0 };
                            }
                        }
                    }
                }
                Event::Mouse(mouse) => {
                    if let crate::app::AppMode::Menu { .. } = app.mode {
                        match mouse.kind {
                            MouseEventKind::ScrollUp => app.select_previous(),
                            MouseEventKind::ScrollDown => app.select_next(),
                            MouseEventKind::Down(_) => {
                                let row = mouse.row as usize;
                                let item_index = row.saturating_sub(MENU_LIST_START_ROW);
                                if item_index < 5 {
                                    app.select_index(item_index);
                                    app.launch_selected();
                                }
                            }
                            _ => {}
                        }
                    }
                }
                Event::Resize(width, height) => {
                    app.resize(width, height);
                }
                _ => {}
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