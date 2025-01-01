use crossterm::{event::KeyCode, terminal};
use ratatui::DefaultTerminal;

pub struct App {
    pub running: bool
}

impl App {
    pub fn new() -> App {
        App {
            running: true
        }
    }

    pub fn on_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.running = false,
            _ => {}
        }
    }
}
