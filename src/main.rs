use std::{
    io::{stdout, Result}, 
    time::Duration
};

use crossterm::{
    event::{self, Event}, 
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

use wi_tui::{app::App, ui::draw_ui};

fn main() -> Result<()> {

    let mut app = App::new();

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;
    
    app.refresh_networks();

    while app.running {
        terminal.draw(|frame| draw_ui(frame, &app))?;

        if event::poll(Duration::from_millis(250))? {
            app.handle_event()?;
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
