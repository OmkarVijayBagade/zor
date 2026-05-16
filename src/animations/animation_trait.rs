use ratatui::Frame;
use crossterm::event::{Event, KeyCode};

pub trait Animation {
    fn init(&mut self, width: u16, height: u16);
    fn update(&mut self, dt: f32);
    fn render(&self, frame: &mut Frame);
    fn resize(&mut self, width: u16, height: u16);
    fn handle_input(&mut self, event: Event) -> bool {
        // Return true if the event was handled and we should not quit
        // By default, we handle 'q' to quit the animation and return to menu
        if let Event::Key(key) = event {
            if key.code == KeyCode::Char('q') {
                return false;
            }
        }
        true
    }
}