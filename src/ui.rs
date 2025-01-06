use ratatui::{
    buffer::Buffer, 
    layout::{Constraint, Direction, Layout, Rect}, 
    style::{Modifier, Style, Stylize}, 
    widgets::{Block, Borders, List, ListItem, Paragraph, Widget}, 
};

use crate::app::App;

impl Widget for &mut App {
    
    fn render(self, area: Rect, buf: &mut Buffer) {
        let chunks = Layout::default()
            .direction(Direction::Vertical)
            .constraints([
                Constraint::Length(3),
                Constraint::Min(0),
            ])
            .split(area);

        // Render the title block
        App::render_header(chunks[0], buf);

        // Render the list block
        self.render_network_list(chunks[1], buf);
    }
    
}


impl App {

    fn render_header(area: Rect, buf: &mut Buffer) {
        Paragraph::new("Wi-Tui")
            .bold()
            .centered()
            .render(area, buf);
    }

    pub fn render_network_list(&self, area: Rect, buf: &mut Buffer) {
        let items: Vec<ListItem> = self
            .networks
            .iter()
            .map(|network| {
                ListItem::new(format!(
                    "{} | Signal: {} | Security: {}",
                    network.ssid, network.signal, network.security
                ))
            })
            .collect();

        let list = List::new(items)
            .block(
                Block::default()
                    .title("Available Networks")
                    .borders(Borders::ALL),
            )
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC));

        list.render(area, buf);
    }
}


