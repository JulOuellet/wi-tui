use crossterm::event::KeyEventKind;
use ratatui::crossterm::event::{
    self, 
    Event,
    KeyEvent,
    KeyCode
};

use crate::app::App;

impl App {

    pub fn handle_event(&mut self) -> Result<(), std::io::Error> {
        if let Event::Key(key) = event::read()? {
            self.handle_key(key);
        }
        Ok(())
    }

    pub fn handle_key(&mut self, key: KeyEvent) {
        if key.kind != KeyEventKind::Press {
            return;
        }

        match key.code {
            KeyCode::Char('q') => self.running = false,
            KeyCode::Char('r') => self.refresh_networks(),
            KeyCode::Up | KeyCode::Char('k') => self.move_selection_up(),
            KeyCode::Down | KeyCode::Char('j') => self.move_selection_down(),
            _ => {}
        }
    }

}
