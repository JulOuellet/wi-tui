use ratatui::{
    buffer::Buffer, 
    layout::{Constraint, Layout, Rect}, 
    style::{Modifier, Style, Stylize}, 
    text::Line, 
    widgets::{Block, BorderType, Borders, List, ListItem, Padding, Paragraph, Widget} 
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
        App::render_body(&mut *self, body_area, buf);
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

    fn render_body(&self, area: Rect, buf: &mut Buffer) {
        let (ssid_width, signal_width, security_width, rate_width, bars_width) =
            self.calculate_column_widths();

        let header = ListItem::new(format!(
                "{:<width_ssid$} | {:<width_signal$} | {:<width_security$} | {:<width_rate$} | {:<width_bars$}",
                "SSID",
                "Signal",
                "Security",
                "Rate",
                "Bars",
                width_ssid = ssid_width,
                width_signal = signal_width,
                width_security = security_width,
                width_rate = rate_width,
                width_bars = bars_width
        ))
            .style(Style::default().bold().fg(ratatui::style::Color::Cyan));

        let network_items: Vec<ListItem> = self
            .networks
            .iter()
            .enumerate()
            .map(|(i, network)| {
                let mut item = ListItem::new(format!(
                        "{:<width_ssid$} | {:<width_signal$} | {:<width_security$} | {:<width_rate$} | {:<width_bars$}",
                        network.ssid,
                        network.signal,
                        network.security,
                        network.rate,
                        network.bars,
                        width_ssid = ssid_width,
                        width_signal = signal_width,
                        width_security = security_width,
                        width_rate = rate_width,
                        width_bars = bars_width
                ));

                if i == self.selected_index {
                    item = item.style(Style::default().add_modifier(Modifier::REVERSED));
                }

                item
            })
        .collect();

        let mut list_items = Vec::with_capacity(network_items.len() + 1);
        list_items.push(header);
        list_items.extend(network_items);

        let list = List::new(list_items)
            .block(
                Block::default()
                .title(" Available Networks ")
                .bold()
                .borders(Borders::ALL)
                .border_type(BorderType::Rounded)
                .padding(Padding::new(3, 3, 1, 1)),
            )
            .highlight_style(Style::default().add_modifier(Modifier::ITALIC));

        list.render(area, buf);
    }

    fn calculate_column_widths(&self) -> (usize, usize, usize, usize, usize) {
        let mut ssid_width = "SSID".len();
        let mut signal_width = "Signal".len();
        let mut security_width = "Security".len();
        let mut rate_width = "Rate".len();
        let mut bars_width = "Bars".len();

        for network in &self.networks {
            ssid_width = ssid_width.max(network.ssid.len());
            signal_width = signal_width.max(network.signal.len());
            security_width = security_width.max(network.security.len());
            rate_width = rate_width.max(network.rate.len());
            bars_width = bars_width.max(network.bars.len());
        }

        (ssid_width, signal_width, security_width, rate_width, bars_width)
    }

}
