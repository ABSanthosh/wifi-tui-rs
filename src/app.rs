use crate::ui::list::Network;
use crate::ui::tab::SelectedTab;

use color_eyre::eyre::{Ok, Result};
use ratatui::layout::Flex;
use ratatui::style::{Color, Style};
use ratatui::widgets::{StatefulWidget, Wrap};
use ratatui::Frame;
use ratatui::{
    buffer::Buffer,
    crossterm::event::{self, Event, KeyCode, KeyEventKind},
    layout::{Constraint, Layout, Rect},
    widgets::{Block, BorderType, Borders, Paragraph, Tabs, Widget},
    DefaultTerminal,
};
use strum::IntoEnumIterator;
use tui_widget_list::ListState;

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

#[derive(Default)]
pub struct GeneralState {
    pub list_state: ListState,
}

impl App {
    pub fn new() -> App {
        App {
            state: AppState::Running,
            selected_tab: SelectedTab::default(),
            known_networks: {
                let mut networks = Vec::new();
                for i in 1..=10 {
                    networks.push(Network {
                        ssid: format!("Known WiFi {}", i),
                        signal_strength: 100 - (i * 2),
                        is_known: false,
                        ip_address: None,
                        is_connected: false,
                        security: "WPA2".to_string(),
                        is_selected: false,
                    });
                }
                networks
            },
            unknown_networks: {
                let mut networks = Vec::new();
                for i in 1..=40 {
                    networks.push(Network {
                        ssid: format!("Unknown WiFi {}", i),
                        signal_strength: 100 - (i * 2),
                        is_known: false,
                        ip_address: None,
                        is_connected: false,
                        security: "WPA2".to_string(),
                        is_selected: false,
                    });
                }
                networks
            },
        }
    }

    pub fn run(mut self, mut terminal: DefaultTerminal) -> Result<()> {
        let mut list_state = GeneralState::default();
        list_state.list_state.select(Some(0)); // Preselect the first item

        while self.state == AppState::Running {
            terminal.draw(|frame: &mut Frame| {
                frame.render_stateful_widget(&self, frame.area(), &mut list_state)
            })?;

            self.handle_keys(&mut list_state)?;
        }
        Ok(())
    }

    fn handle_keys(&mut self, state: &mut GeneralState) -> Result<()> {
        if let Event::Key(key) = event::read()? {
            if key.kind == KeyEventKind::Press {
                match key.code {
                    KeyCode::Char('l') | KeyCode::Right => {
                        self.selected_tab = self.selected_tab.next(state);
                    }
                    KeyCode::Char('h') | KeyCode::Left => {
                        self.selected_tab = self.selected_tab.previous(state);
                    }
                    KeyCode::Char('j') | KeyCode::Down => state.list_state.next(),
                    KeyCode::Char('k') | KeyCode::Up => state.list_state.previous(),
                    KeyCode::Char('q') | KeyCode::Esc => self.state = AppState::Quitting,
                    _ => {}
                }
            }
        }
        Ok(())
    }

    pub fn render_title(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("wifi-tui")
            .block(Block::default().borders(Borders::TOP))
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }

    pub fn render_footer(&self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Use ← → | ▲ ▼ | q to quit")
            .centered()
            .wrap(Wrap { trim: false })
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
impl StatefulWidget for &App {
    type State = GeneralState;
    fn render(self, area: Rect, buf: &mut Buffer, state: &mut Self::State) {
        use Constraint::{Length, Max, Min};

        let outer_layout = Layout::horizontal([Length(1), Max(70), Length(1)]).flex(Flex::Center);
        let [_, centered_area, _] = outer_layout.areas(area);

        let inner_layout = Layout::vertical([
            Length(2), // header area
            Min(0),    // content area
            Length(1), // footer
        ]);
        let [header_area, inner_area, footer_area] = inner_layout.areas(centered_area);

        self.render_tabs(header_area, buf);
        self.render_footer(footer_area, buf);

        match self.selected_tab {
            SelectedTab::Known => {
                self.render_list(inner_area, buf, &self.known_networks, &mut state.list_state);
            }
            SelectedTab::Unknown => {
                self.render_list(
                    inner_area,
                    buf,
                    &self.unknown_networks,
                    &mut state.list_state,
                );
            }
            SelectedTab::Radio => {
                self.selected_tab.render_radio(inner_area, buf);
            }
        }
    }
}
