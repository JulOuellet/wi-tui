use ratatui::{
    layout::{Constraint, Direction, Layout},
    style::{Modifier, Style},
    widgets::{Block, Borders, List, ListItem},
    Frame,
};
use crate::app::App;

pub fn draw_ui(frame: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .constraints(
            [
                Constraint::Length(3),
                Constraint::Min(0),
            ]
            .as_ref(),
        )
        .split(frame.area());

    let title = Block::default()
        .title(" Wi-Tui ")
        .borders(Borders::ALL)
        .style(Style::default().add_modifier(Modifier::BOLD));
    frame.render_widget(title, chunks[0]);

    let items: Vec<ListItem> = app.networks
        .iter()
        .map(|network| {
            ListItem::new(format!(
                "{} | Signal: {} | Security: {}",
                network.ssid, network.signal, network.security
            ))
        })
        .collect();

    let list = List::new(items)
        .block(Block::default().title("Available Networks").borders(Borders::ALL))
        .highlight_style(Style::default().add_modifier(Modifier::ITALIC));
    frame.render_widget(list, chunks[1]);
}



