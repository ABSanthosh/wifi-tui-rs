use ratatui::{buffer::Buffer, layout::Rect, symbols, widgets::{Block, Borders, Padding, Paragraph, Widget, Wrap}};

use crate::app::App;

pub struct Network {
    pub ssid: String,
    pub signal_strength: u8,
    pub is_known: bool,
    pub ip_address: Option<String>,
    pub is_connected: bool,
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

    pub fn render_list(&self, area: Rect, buf: &mut Buffer, networks: &[Network]) {
        let list_items: Vec<_> = networks
            .iter()
            .map(|network| {
                let signal_strength = network.signal_strength;
                let ssid = &network.ssid;
                format!("{} ({}%)", ssid, signal_strength)
            })
            .collect();

        Paragraph::new(list_items.join("\n"))
            // .block(Block::default().borders(Borders::ALL))
            .block(self.container())
            .wrap(Wrap { trim: true })
            .render(area, buf);
    }
}
