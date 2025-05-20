use ratatui::{
    buffer::Buffer,
    layout::{Alignment, Constraint, Flex, Layout, Rect},
    prelude::StatefulWidget,
    style::{Color, Style},
    symbols::{self},
    text::Line,
    widgets::{Block, Borders, Padding, Widget},
};
use tui_widget_list::ListState;

use crate::app::App;

// TODO: in the future use 16bit colors and let user pass palette like catppuccine
const PINK: Color = Color::Rgb(255, 105, 179); // #FF69B3
const DARK_PINK: Color = Color::Rgb(177, 94, 135); // #b15e87
const BLUE: Color = Color::Rgb(52, 152, 219); // #3498DB

#[derive(Clone)]
pub struct Network {
    pub ssid: String,
    pub signal_strength: u8,
    pub is_known: bool,
    pub ip_address: Option<String>,
    pub is_connected: bool,
    pub security: String,
    pub is_selected: bool,
}

enum BarStyle {
    BlockBars,
    BinaryCircles,
    BinarySquares,
    Stars,
    MiniBars,
}

impl Network {
    fn get_ssid(&self) -> String {
        if self.is_connected {
            format!("* {}", self.ssid)
        } else {
            self.ssid.clone()
        }
    }

    fn generate_border(&self, is_selected: bool) -> Block<'static> {
        Block::bordered()
            .border_set(symbols::border::Set {
                horizontal_bottom: "╴",
                bottom_left: " ",
                vertical_left: symbols::line::THICK.vertical,
                ..symbols::border::PLAIN
            })
            .border_style(if is_selected {
                Style::default().fg(BLUE)
            } else {
                Style::default()
            })
            .borders(Borders::BOTTOM | Borders::TOP)
    }

    fn signal_strength_bar(&self, strength: u8, style: BarStyle) -> String {
        let strength = strength.min(100); // Clamp to 100
        let total = 5;
        let filled = strength as usize * total / 100;
        let empty = total - filled;

        match style {
            BarStyle::BlockBars => match strength {
                0..=20 => "▂".to_string(),
                21..=40 => "▂▄".to_string(),
                41..=60 => "▂▄▆".to_string(),
                61..=80 => "▂▄▆█".to_string(),
                _ => "▂▄▆██".to_string(),
            },
            BarStyle::BinaryCircles => format!("[{}{}]", "●".repeat(filled), "○".repeat(empty)),
            BarStyle::BinarySquares => format!("[{}{}]", "■".repeat(filled), "□".repeat(empty)),
            BarStyle::MiniBars => format!("[{}{}]", "█".repeat(filled), "░".repeat(empty)),
            BarStyle::Stars => format!("[{}{}]", "*".repeat(filled), "-".repeat(empty)),
        }
    }
}

impl Widget for Network {
    fn render(self, area: Rect, buf: &mut Buffer) {
        let selected_item_style: (Style, Style) = if self.is_selected {
            let primary = Style::default().fg(PINK);
            let secondary = Style::default().fg(DARK_PINK);
            (primary, secondary)
        } else {
            let primary = Style::default();
            let secondary = Style::default().fg(Color::DarkGray);
            (primary, secondary)
        };

        let ssid_line = Line::styled(self.get_ssid(), selected_item_style.0);
        let security_line = Line::styled(&self.security, selected_item_style.1);
        let bars = Line::styled(
            self.signal_strength_bar(self.signal_strength, BarStyle::BinaryCircles),
            selected_item_style.0,
        )
        .alignment(Alignment::Right);

        let signal_text = Line::styled(
            format!("({}%)", self.signal_strength),
            selected_item_style.1,
        )
        .alignment(Alignment::Right);

        // Overall layout
        let [first_row, second_row, bottom_border] = Layout::vertical([
            Constraint::Length(1), // SSID area
            Constraint::Length(1), // Security area
            Constraint::Length(1), // border
        ])
        .areas(area);

        // Layout for first row
        let [ssid_area, bar_area] =
            Layout::horizontal([Constraint::Percentage(100), Constraint::Min(10)])
                .flex(Flex::SpaceBetween)
                .areas(first_row);

        ssid_line.render(ssid_area, buf);
        bars.render(bar_area, buf);

        // Layout for second row
        let [security_area, signal_area] =
            Layout::horizontal([Constraint::Percentage(100), Constraint::Length(10)])
                .flex(Flex::SpaceBetween)
                .areas(second_row);

        self.generate_border(self.is_selected)
            .render(bottom_border, buf);
        security_line.render(security_area, buf);
        signal_text.render(signal_area, buf);
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
            let mut item: Network = networks[ctx.index].clone();

            if ctx.is_selected {
                item.is_selected = true;
            } else {
                item.is_selected = false;
            }

            // The number tells how many lines the item will take
            (item, 3)
        });

        ListView::new(builder, networks.len())
            .scroll_padding(2)
            .block(self.container())
            .render(area, buf, list_state);
    }
}
