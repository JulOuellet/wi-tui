use crossterm::event::KeyCode;

use crate::networks::{get_networks, Network};

pub struct App {
    pub running: bool,
    pub networks: Vec<Network>
}

impl App {
    pub fn new() -> App {
        App {
            running: true,
            networks: vec![]
        }
    }

    pub fn on_key(&mut self, key: KeyCode) {
        match key {
            KeyCode::Char('q') => self.running = false,
            KeyCode::Char('r') => self.refresh_networks(),
            _ => {}
        }
    }

    pub fn refresh_networks(&mut self) {
        if let Ok(networks) = get_networks() {
            self.networks = networks;
            self.sort_networks();
        }
    }

    pub fn sort_networks(&mut self) {
        self.networks.sort_by(|a, b| b.signal.cmp(&a.signal));
    }
}
