use crate::list::Network;
use crate::ui::tab::SelectedTab;
use color_eyre::eyre::{Ok, Result};
use ratatui::style::{Color, Style};
use ratatui::symbols;
use ratatui::text::Line;
use ratatui::widgets::{Padding, Wrap};
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs, Widget},
    DefaultTerminal,
};
use strum::IntoEnumIterator;

pub struct App {
    state: AppState,
    selected_tab: SelectedTab,
    known_networks: Vec<Network>,
    unknown_networks: Vec<Network>,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

impl App {
    pub fn new() -> App {
        App {
            state: AppState::Running,
            selected_tab: SelectedTab::default(),
            known_networks: vec![
                Network {
                    ssid: "Home WiFi".to_string(),
                    signal_strength: 75,
                    is_known: true,
                    ip_address: None,
                    is_connected: true,
                },
                Network {
                    ssid: "Office WiFi".to_string(),
                    signal_strength: 50,
                    is_known: true,
                    ip_address: None,
                    is_connected: false,
                },
                Network {
                    ssid: "Coffee Shop WiFi".to_string(),
                    signal_strength: 30,
                    is_known: true,
                    ip_address: None,
                    is_connected: false,
                },
                Network {
                    ssid: "Public WiFi".to_string(),
                    signal_strength: 20,
                    is_known: true,
                    ip_address: None,
                    is_connected: false,
                },
            ],
            unknown_networks: vec![
                Network {
                    ssid: "Unknown WiFi 1".to_string(),
                    signal_strength: 80,
                    is_known: false,
                    ip_address: None,
                    is_connected: false,
                },
                Network {
                    ssid: "Unknown WiFi 2".to_string(),
                    signal_strength: 60,
                    is_known: false,
                    ip_address: None,
                    is_connected: false,
                },
            ],
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        while self.state == AppState::Running {
            terminal.draw(|frame| frame.render_widget(&self, frame.area()))?;
            self.handle_keys()?;
        }
        Ok(())
    }

    fn handle_keys(&mut self) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('l') | KeyCode::Right => self.next_tab(),
                    KeyCode::Char('h') | KeyCode::Left => self.previous_tab(),
                    KeyCode::Char('j') | KeyCode::Down => {} // TODO: Implement list navigation
                    KeyCode::Char('k') | KeyCode::Up => {}   // TODO: Implement list navigation
                    KeyCode::Char('q') | KeyCode::Esc => self.quit(),
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn next_tab(&mut self) {
        self.selected_tab = self.selected_tab.next();
    }

    pub fn previous_tab(&mut self) {
        self.selected_tab = self.selected_tab.previous();
    }

    pub fn quit(&mut self) {
        self.state = AppState::Quitting;
    }

    pub fn render_title(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("wifi-tui")
            .block(Block::default().borders(Borders::TOP))
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }

    pub fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Line::raw("Use ← → to change tab | ↓↑ to move | Press q to quit")
            .centered()
            .render(area, buf);
    }

    pub fn render_tabs(&self, area: Rect, buf: &mut Buffer) {
        let titles = SelectedTab::iter().map(|tab| tab.to_string());
        let selected_tab_index = self.selected_tab as usize;

        let tab_container = Block::bordered()
            .border_type(BorderType::Plain)
            .borders(Borders::TOP | Borders::LEFT | Borders::RIGHT)
            .title("wifi-tui");

        Tabs::new(titles)
            .highlight_style(Style::default().fg(Color::Yellow))
            .select(selected_tab_index)
            .block(tab_container)
            .render(area, buf);
    }

    
}

// This is where the whole app is rendered
impl Widget for &App {
    fn render(self, area: Rect, buf: &mut Buffer) {
        use Constraint::{Length, Min};

        let vertical = Layout::vertical([
            Length(2), // header area
            Min(0),    // content area
            Length(1), // footer
        ]);
        let [header_area, inner_area, footer_area] = vertical.areas(area);

        self.render_tabs(header_area, buf);
        // self.selected_tab.render(inner_area, buf);
        self.render_footer(footer_area, buf);
        // self.render_title(header_area, buf);
        self.render_list(
            inner_area,
            buf,
            match self.selected_tab {
                SelectedTab::Known => &self.known_networks,
                SelectedTab::Unknown => &self.unknown_networks,
            },
        );
    }
}
