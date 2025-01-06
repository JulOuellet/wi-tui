use ratatui::{
    buffer::Buffer, 
    layout::{Constraint, Layout, Rect}, 
    style::{Modifier, Style, Stylize}, 
    text::Line, 
    widgets::{Block, BorderType, Borders, List, ListItem, Paragraph, Widget} 
};

use crate::app::App;

impl Widget for &mut App {

    fn render(self, area: Rect, buf: &mut Buffer) {
        let [header_area, body_area] = Layout::vertical([
            Constraint::Length(6),
            Constraint::Fill(1),
        ])
        .areas(area);

        App::render_header(header_area, buf);
        App::render_network_list(&mut *self, body_area, buf);
    }

}

impl App {

    fn render_header(area: Rect, buf: &mut Buffer) {
        let title_art = vec![
            Line::raw(" __      __.___        _______________ ___.___ "),
            Line::raw("/  \\    /  \\   |       \\__    ___/    |   \\   |"),
            Line::raw("\\   \\/\\/   /   |  ______ |    |  |    |   /   |"),
            Line::raw(" \\        /|   | /_____/ |    |  |    |  /|   |"),
            Line::raw("  \\__/\\  / |___|         |____|  |______/ |___|"),
            Line::raw("       \\/                                      "),
        ];

        let title: Vec<Line> = title_art
            .iter()
            .map(|line| line
                .clone().style(Style::default().bold().fg(ratatui::style::Color::Cyan)))
            .collect();

        Paragraph::new(title)
            .alignment(ratatui::layout::Alignment::Center)
            .render(area, buf);
    }

    pub fn render_network_list(&self, area: Rect, buf: &mut Buffer) {
        let list_items: Vec<ListItem> = self
            .networks
            .iter()
            .enumerate()
            .map(|(i, network)| {
                let mut item = ListItem::new(format!(
                        "{} | Signal: {} | Security: {}",
                        network.ssid, network.signal, network.security
                ));

                if i == self.selected_index {
                    item = item.style(Style::default().add_modifier(Modifier::REVERSED));
                }

                item
            })
            .collect();

        let list = List::new(list_items)
            .block(
                Block::default()
                .title(" Available Networks ")
                .bold()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded))
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC));

        list.render(area, buf);
    }

}

