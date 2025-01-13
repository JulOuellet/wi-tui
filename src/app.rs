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
    pub running: bool,          // state of the application
    pub networks: Vec<Network>, // list of networks
    pub selected_index: usize,  // index of the selected network in the list
    pub scroll_offset: usize,   // offset for scrolling the list of networks
    pub visible_items: usize,   // number of visible list items in the terminal
    pub items_list_offset: u16, // magic number, offset at which the list of items starts
}

impl App {

    pub fn new() -> App {
        App {
            running: true,
            networks: vec![],
            selected_index: 0,
            scroll_offset: 0,
            visible_items: 0,
            items_list_offset: 11,
        }
    }

    pub fn run(&mut self) -> Result<()> {
        self.refresh_networks();
        enable_raw_mode()?;
        stdout().execute(EnterAlternateScreen)?;
        
        let backend = CrosstermBackend::new(stdout());
        let mut terminal = Terminal::new(backend)?;

        loop {
            let terminal_size = terminal.size()?;
            self.visible_items = terminal_size.height.saturating_sub(self.items_list_offset) as usize;

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
            
            if self.selected_index < self.scroll_offset {
                self.scroll_offset = self.selected_index;
            }
        }
    }

    pub fn move_selection_down(&mut self) {
        if self.selected_index < self.networks.len().saturating_sub(1) {
            self.selected_index += 1;
        }

        if self.selected_index >= self.scroll_offset + self.visible_items {
            self.scroll_offset = self.selected_index + 1 - self.visible_items;
        }
    }
}
