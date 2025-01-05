use std::{
    io::{stdout, Result},
    time::Duration
};
use crossterm::{
    event,
    terminal::{
        disable_raw_mode,
        enable_raw_mode,
        EnterAlternateScreen,
        LeaveAlternateScreen
    },
    ExecutableCommand
};

use ratatui::{
    backend::CrosstermBackend,
    Terminal
};

use crate::networks::{get_networks, Network};
use crate::ui::draw_ui;




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

    pub fn run(&mut self) -> Result<()> {
        self.refresh_networks();
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|frame| draw_ui(frame, &self))?;

            if event::poll(Duration::from_millis(250))? {
                self.handle_event()?;
            }

            if !self.running {
                disable_raw_mode()?;
                stdout().execute(LeaveAlternateScreen)?;
                break;
            }
        }

        Ok(())
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
