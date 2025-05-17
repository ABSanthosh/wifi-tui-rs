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
use strum::{Display, EnumIter, FromRepr, IntoEnumIterator};

pub struct App {
    state: AppState,
    selected_tab: SelectedTab,
}

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
enum SelectedTab {
    #[default]
    #[strum(to_string = "known")]
    Known,
    #[strum(to_string = "unknown")]
    Unknown,
}

#[derive(Default, Clone, Copy, PartialEq, Eq)]
enum AppState {
    #[default]
    Running,
    Quitting,
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }
}

impl App {
    pub fn new() -> App {
        App {
            state: AppState::Running,
            selected_tab: SelectedTab::default(),
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
        Line::raw("◄ ► to change tab | Press q to quit")
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
        self.selected_tab.render(inner_area, buf);
        self.render_footer(footer_area, buf);
    }
}

impl SelectedTab {
    fn render_known(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("List of Known wifi networks")
            .block(self.container())
            .render(area, buf);
    }

    fn render_unknown(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("List of Unknown wifi networks")
            .block(self.container())
            .render(area, buf);
    }

    fn container(self) -> Block<'static> {
        // https://ratatui.rs/recipes/layout/collapse-borders/
        let collapsed_border_set = symbols::border::Set {
            top_left: symbols::line::NORMAL.vertical_right,
            top_right: symbols::line::NORMAL.vertical_left,
            ..symbols::border::PLAIN
        };

        Block::bordered()
            .borders(Borders::ALL)
            .border_set(collapsed_border_set)
            .padding(Padding::horizontal(1))
    }
}

impl Widget for SelectedTab {
    fn render(self, area: Rect, buf: &mut Buffer) {
        match self {
            SelectedTab::Known => self.render_known(area, buf),
            SelectedTab::Unknown => self.render_unknown(area, buf),
        }
    }
}
