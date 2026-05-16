use crate::animations::animation_trait::Animation;
use ratatui::{
    style::{Color, Style},
    widgets::{Block, Borders, Widget},
    Frame,
};

pub struct Wave {
    width: u16,
    height: u16,
    t: f32,
}

impl Wave {
    pub fn new() -> Self {
        Wave {
            width: 0,
            height: 0,
            t: 0.0,
        }
    }

    fn compute_y(&self, x: f32) -> (u16, char, Color) {
        let y_wave = 3.0 * (0.05 * x + 1.0 * self.t).sin()
            + 2.0 * (0.1 * x + 1.5 * self.t + 1.0).sin()
            + 1.0 * (0.2 * x + 2.0 * self.t + 2.0).sin();

        let half_h = self.height as f32 / 2.0;
        let y_screen = (half_h + y_wave).round() as i32;
        let y_screen = y_screen.max(0).min((self.height - 1) as i32) as u16;

        let (ch, color) = if y_wave > 2.0 {
            ('~', Color::White)
        } else if y_wave > -1.0 {
            ('-', Color::Cyan)
        } else {
            ('.', Color::Blue)
        };

        (y_screen, ch, color)
    }
}

impl Animation for Wave {
    fn init(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
        self.t = 0.0;
    }

    fn update(&mut self, dt: f32) {
        self.t += dt;
    }

    fn render(&self, frame: &mut Frame) {
        let area = frame.size();
        let block = Block::default().borders(Borders::NONE);
        block.render(area, frame.buffer_mut());

        let buf = frame.buffer_mut();
        let render_width = self.width.min(area.width);
        let mut prev_y: Option<u16> = None;

        for x in 0..render_width {
            let (y_screen, ch, color) = self.compute_y(x as f32);

            buf.set_string(x, y_screen, &ch.to_string(), Style::default().fg(color));

            if let Some(py) = prev_y {
                let start = py.min(y_screen) + 1;
                let end = py.max(y_screen);
                let fill_style = Style::default().fg(Color::DarkGray);
                for fill_y in start..=end {
                    if fill_y < self.height {
                        buf.set_string(x, fill_y, " ", fill_style);
                    }
                }
            }

            prev_y = Some(y_screen);
        }
    }

    fn resize(&mut self, width: u16, height: u16) {
        self.width = width;
        self.height = height;
    }
}