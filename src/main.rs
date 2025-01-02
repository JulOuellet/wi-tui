use std::{io::{stdout, Result}, time::Duration};

use crossterm::{event::{self, Event}, terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen}, ExecutableCommand};
use ratatui::{
    backend::CrosstermBackend, layout::Alignment, style::{Modifier, Style}, widgets::{Block, BorderType, Borders, Paragraph}, Terminal
};
use wi_tui::app::App;

fn main() -> Result<()> {

    enable_raw_mode()?;
    stdout().execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout());
    let mut terminal = Terminal::new(backend)?;

    let mut app = App::new();

    while app.running {
        terminal.draw(|frame| {
            let area = frame.area();
            frame.render_widget(
                Paragraph::new("")
                    .alignment(Alignment::Center)
                    .block(Block::default()
                        .title(" Wi-Tui ")
                        .title_alignment(Alignment::Center)
                        .borders(Borders::ALL)
                        .style(Style::default().add_modifier(Modifier::BOLD))
                        .border_type(BorderType::Rounded)),
                area
            );
        })?;

        if event::poll(Duration::from_millis(250))? {
            if let Event::Key(key) = event::read()? {
                app.on_key(key.code);
            }
        }
    }

    disable_raw_mode()?;
    stdout().execute(LeaveAlternateScreen)?;

    Ok(())
}
