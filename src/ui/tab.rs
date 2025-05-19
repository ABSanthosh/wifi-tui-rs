use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};
use strum::{Display, EnumIter, FromRepr};

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "known")]
    Known,
    #[strum(to_string = "unknown")]
    Unknown,
}

impl SelectedTab {
    /// Get the previous tab, if there is no previous tab return the current tab.
    pub fn previous(self) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);
        Self::from_repr(previous_index).unwrap_or(self)
    }

    /// Get the next tab, if there is no next tab return the current tab.
    pub fn next(self) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);
        Self::from_repr(next_index).unwrap_or(self)
    }

    pub fn _render_known(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("List of Known wifi networks")
            .block(self._container())
            .render(area, buf);
    }

    pub fn _render_unknown(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("List of Unknown wifi networks")
            .block(self._container())
            .render(area, buf);
    }

    fn _container(self) -> Block<'static> {
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
}

// impl Widget for SelectedTab {
//     fn render(self, area: Rect, buf: &mut Buffer) {
//         match self {
//             SelectedTab::Known => self.render_known(area, buf),
//             SelectedTab::Unknown => self.render_unknown(area, buf),
//         }
//     }
// }
