use ratatui::{
    buffer::Buffer,
    layout::{Constraint, Layout, Rect},
    prelude::StatefulWidget,
    style::{Color, Style, Styled},
    symbols::{self},
    text::Line,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};
use tui_widget_list::ListState;

use crate::app::App;

const PINK: Color = Color::Rgb(255, 105, 179); // #FF69B3
const BLUE: Color = Color::Rgb(52, 152, 219); // #3498DB

#[derive(Clone)]
pub struct Network {
    pub ssid: String,
    pub signal_strength: u8,
    pub is_known: bool,
    pub ip_address: Option<String>,
    pub is_connected: bool,
    pub security: String,
    pub style: Style,
}

impl Widget for Network {
    fn render(self, area: Rect, buf: &mut Buffer) {
        // Layout: horizontally split into left (SSID) and right (Signal %)
        let chunks = Layout::horizontal([Constraint::Min(20), Constraint::Length(10)]).split(area);

        // Top line: "* SSID" or "SSID"
        let ssid_text = if self.is_connected {
            format!("* {}", self.ssid)
        } else {
            self.ssid.clone()
        };

        let ssid_line = Line::styled(ssid_text, Style::default().fg(Color::White));
        let security_line = Line::styled(self.security, Style::default().fg(Color::DarkGray));

        // let ssid_line = Line::styled(ssid_text, self.style.fg(Color::White));
        // let security_line = Line::styled("WPA2", self.style.fg(Color::DarkGray));

        let dashed_border_set = symbols::border::Set {
            horizontal_bottom: "╴",
            ..symbols::border::PLAIN
        };

        let ssid_block = Paragraph::new(vec![ssid_line, security_line]).block(
            Block::bordered()
                .borders(Borders::BOTTOM)
                .border_set(dashed_border_set),
        );

        let signal_text = format!("({}%)", self.signal_strength);
        let bars = signal_strength_bar(self.signal_strength);
        let signal_block = Paragraph::new(vec![Line::from(bars), Line::from(signal_text)])
            .alignment(ratatui::layout::Alignment::Right)
            .block(
                Block::bordered()
                    .borders(Borders::BOTTOM)
                    .border_set(dashed_border_set),
            );

        ssid_block.render(chunks[0], buf);
        signal_block.render(chunks[1], buf);
    }
}

fn signal_strength_bar(strength: u8) -> &'static str {
    match strength {
        0..=20 => "▂",
        21..=40 => "▂▄",
        41..=60 => "▂▄▆",
        61..=80 => "▂▄▆█",
        _ => "▂▄▆██",
    }
}

impl Styled for Network {
    type Item = Self;

    fn style(&self) -> Style {
        self.style
    }

    fn set_style<S: Into<Style>>(mut self, style: S) -> Self::Item {
        self.style = style.into();
        self
    }
}

impl App {
    fn container(&self) -> Block<'static> {
        // Ref: https://ratatui.rs/recipes/layout/collapse-borders/
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

    pub fn render_list(
        &self,
        area: Rect,
        buf: &mut Buffer,
        networks: &[Network],
        list_state: &mut ListState,
    ) {
        use tui_widget_list::{ListBuilder, ListView};

        let builder = ListBuilder::new(|ctx| {
            let mut item = networks[ctx.index].clone();

            if ctx.is_selected {
                item.style = Style::default()
                    .bg(Color::Rgb(255, 153, 0)) // bright orange background
                    .fg(Color::Black); // dark text
            } else {
                item.style = Style::default(); // fallback/default style
            }

            (item, 3)
        });

        ListView::new(builder, networks.len())
            .scroll_padding(2)
            .block(self.container())
            .render(area, buf, list_state);
    }

    // pub fn render_list(&self, area: Rect, buf: &mut Buffer, networks: &[Network]) {
    //     let list_items: Vec<_> = networks
    //         .iter()
    //         .map(|network| {
    //             let signal_strength = network.signal_strength;
    //             let ssid = &network.ssid;
    //             format!("{} ({}%)", ssid, signal_strength)
    //         })
    //         .collect();

    //     Paragraph::new(list_items.join("\n"))
    //         // .block(Block::default().borders(Borders::ALL))
    //         .block(self.container())
    //         .wrap(Wrap { trim: true })
    //         .render(area, buf);
    // }
}
