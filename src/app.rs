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

pub struct App {
    pub running: bool,
    pub networks: Vec<Network>,
    pub selected_index: usize
}

impl App {

    pub fn new() -> App {
        App {
            running: true,
            networks: vec![],
            selected_index: 0
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.refresh_networks();
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;

        loop {
            terminal.draw(|frame| frame.render_widget(&mut *self, frame.area()))?;

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

    pub fn move_selection_up(&mut self) {
        if self.selected_index > 0 {
            self.selected_index -= 1;
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.selected_index < self.networks.len().saturating_sub(1) {
            self.selected_index += 1;
        }
    }
}
