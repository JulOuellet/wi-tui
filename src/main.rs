use std::io;
use tokio::{
    sync::mpsc,
    time::{sleep, Duration},
};
use ratatui::{
    backend::CrosstermBackend,
    widgets::{Block, Borders, List, ListItem, Paragraph, Clear},
    Terminal, Frame,
    layout::{Layout, Constraint, Direction, Rect, Alignment},
    style::{Style, Color},
};
use crossterm::{
    event::{self, Event as CrosstermEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen},
    ExecutableCommand,
};

// Custom events for our application
enum Event {
    Key(CrosstermEvent),
    Tick,
    NetworkScan(Vec<Network>),
    // Add more events as needed:
    // ConnectionStateChanged(ConnectionState),
    // SignalStrengthUpdate(String, u8),
}

#[derive(PartialEq)]
enum InputMode {
    Normal,
    Popup,
}

struct App {
    networks: Vec<Network>,
    selected: Option<usize>,
    input_mode: InputMode,
    scanning: bool,
}

#[derive(Clone)]
struct Network {
    ssid: String,
    signal_strength: u8,
}

impl App {
    fn new() -> App {
        App {
            networks: Vec::new(),
            selected: None,
            input_mode: InputMode::Normal,
            scanning: false,
        }
    }

    fn next(&mut self) {
        let i = match self.selected {
            Some(i) => {
                if i >= self.networks.len() - 1 {
                    0
                } else {
                    i + 1
                }
            }
            None => 0,
        };
        self.selected = Some(i);
    }

    fn previous(&mut self) {
        let i = match self.selected {
            Some(i) => {
                if i == 0 {
                    self.networks.len() - 1
                } else {
                    i - 1
                }
            }
            None => 0,
        };
        self.selected = Some(i);
    }
}

// Spawn an event handler that will send events through a channel
fn spawn_event_handler() -> mpsc::Receiver<Event> {
    let (tx, rx) = mpsc::channel(100);
    let event_tx = tx.clone();

    // Handle keyboard events
    tokio::spawn(async move {
        loop {
            if let Ok(event) = event::read() {
                let _ = event_tx.send(Event::Key(event)).await;
            }
        }
    });

    // Handle timer events (for UI updates)
    let tick_tx = tx.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_millis(100)).await;
            let _ = tick_tx.send(Event::Tick).await;
        }
    });

    // Simulate network scanning (replace with real network scanning later)
    let scan_tx = tx.clone();
    tokio::spawn(async move {
        loop {
            sleep(Duration::from_secs(5)).await; // Scan every 5 seconds
            let networks = vec![
                Network { ssid: String::from("Home_WiFi"), signal_strength: 90 },
                Network { ssid: String::from("Neighbor's_Network"), signal_strength: 65 },
                Network { ssid: String::from("Coffee_Shop"), signal_strength: 45 },
                Network { ssid: String::from("Guest_Network"), signal_strength: 80 },
                Network { ssid: String::from("Free_WiFi"), signal_strength: 30 },
            ];
            let _ = scan_tx.send(Event::NetworkScan(networks)).await;
        }
    });

    rx
}

#[tokio::main]
async fn main() -> io::Result<()> {
    // Terminal initialization
    enable_raw_mode()?;
    let mut stdout = io::stdout();
    stdout.execute(EnterAlternateScreen)?;
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;

    // Setup app and run it
    let mut app = App::new();
    let events = spawn_event_handler();
    let res = run_app(&mut terminal, &mut app, events).await;

    // Restore terminal
    disable_raw_mode()?;
    terminal.backend_mut().execute(LeaveAlternateScreen)?;
    terminal.show_cursor()?;

    if let Err(err) = res {
        println!("{err:?}");
    }

    Ok(())
}

async fn run_app<B: ratatui::backend::Backend>(
    terminal: &mut Terminal<B>,
    app: &mut App,
    mut events: mpsc::Receiver<Event>,
) -> io::Result<()> {
    loop {
        terminal.draw(|f| ui(f, app))?;

        match events.recv().await {
            Some(Event::Key(event)) => {
                if let CrosstermEvent::Key(key) = event {
                    match app.input_mode {
                        InputMode::Normal => match key.code {
                            KeyCode::Char('q') => return Ok(()),
                            KeyCode::Down => app.next(),
                            KeyCode::Up => app.previous(),
                            KeyCode::Enter => {
                                if app.selected.is_some() {
                                    app.input_mode = InputMode::Popup;
                                }
                            }
                            KeyCode::Char('r') => {
                                app.scanning = true;
                                // Here you would trigger a new network scan
                            }
                            _ => {}
                        },
                        InputMode::Popup => match key.code {
                            KeyCode::Esc => app.input_mode = InputMode::Normal,
                            _ => {}
                        },
                    }
                }
            }
            Some(Event::Tick) => {
                // Handle timer tick - could update UI animations, etc.
            }
            Some(Event::NetworkScan(networks)) => {
                app.networks = networks;
                app.scanning = false;
                if app.selected.is_none() && !app.networks.is_empty() {
                    app.selected = Some(0);
                }
            }
            None => return Ok(()),
        }
    }
}

// UI rendering functions remain mostly the same
fn ui(f: &mut Frame, app: &App) {
    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(3),
            Constraint::Min(1),
        ])
        .split(f.size());

    let title = Block::default()
        .title(if app.scanning {
            "WiFi Networks (Scanning...)"
        } else {
            "WiFi Networks (Press 'r' to refresh)"
        })
        .borders(Borders::ALL);
    f.render_widget(title, chunks[0]);

    let items: Vec<ListItem> = app
        .networks
        .iter()
        .enumerate()
        .map(|(i, network)| {
            let signal_bars = match network.signal_strength {
                80..=100 => "▂▄▆█",
                60..=79 => "▂▄▆ ",
                40..=59 => "▂▄  ",
                20..=39 => "▂   ",
                _ => "    ",
            };
            
            ListItem::new(format!("{} {} ({}%)", 
                network.ssid, signal_bars, network.signal_strength))
                .style(Style::default().fg(if Some(i) == app.selected {
                    Color::Yellow
                } else {
                    Color::White
                }))
        })
        .collect();

    let networks = List::new(items)
        .block(Block::default().borders(Borders::ALL))
        .highlight_style(Style::default().fg(Color::Yellow));
    
    f.render_widget(networks, chunks[1]);

    if app.input_mode == InputMode::Popup {
        let network = app.selected.and_then(|i| app.networks.get(i));
        if let Some(network) = network {
            render_popup(f, network);
        }
    }
}

// render_popup function remains the same as before
fn render_popup(f: &mut Frame, network: &Network) {
    let area = f.size();
    let popup_layout = Layout::default()
        .direction(Direction::Vertical)
        .constraints([
            Constraint::Length((area.height - 12).max(0) / 2),
            Constraint::Length(12),
            Constraint::Min(0),
        ])
        .split(area);

    let popup_width = (area.width as f32 * 0.6) as u16;
    let popup_area = Layout::default()
        .direction(Direction::Horizontal)
        .constraints([
            Constraint::Length((area.width - popup_width).max(0) / 2),
            Constraint::Length(popup_width),
            Constraint::Min(0),
        ])
        .split(popup_layout[1])[1];

    f.render_widget(Clear, popup_area);
    f.render_widget(
        Block::default()
            .title("Network Details")
            .borders(Borders::ALL)
            .style(Style::default().bg(Color::Black)),
        popup_area,
    );

    let inner_area = Layout::default()
        .direction(Direction::Vertical)
        .margin(1)
        .constraints([
            Constraint::Length(2),
            Constraint::Length(2),
            Constraint::Length(2),
        ])
        .split(popup_area);

    let network_name = Paragraph::new(format!("Network: {}", network.ssid))
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Left);
    f.render_widget(network_name, inner_area[0]);

    let signal_info = Paragraph::new(format!("Signal Strength: {}%", network.signal_strength))
        .style(Style::default().fg(Color::Yellow))
        .alignment(Alignment::Left);
    f.render_widget(signal_info, inner_area[1]);

    let instructions = Paragraph::new("Press ESC to close")
        .style(Style::default().fg(Color::Gray))
        .alignment(Alignment::Center);
    f.render_widget(instructions, inner_area[2]);
}
