use ratatui::{
    buffer::Buffer,
    layout::Rect,
    symbols,
    widgets::{Block, Borders, Padding, Paragraph, Widget},
};
use strum::{Display, EnumIter, FromRepr};

use crate::app::GeneralState;

#[derive(Default, Clone, Copy, Display, FromRepr, EnumIter, PartialEq)]
pub enum SelectedTab {
    #[default]
    #[strum(to_string = "known")]
    Known,
    #[strum(to_string = "unknown")]
    Unknown,
    #[strum(to_string = "radio")]
    Radio,
}

impl SelectedTab {
    // Get the previous tab, if there is no previous tab return the current tab.
    pub fn previous(self, state: &mut GeneralState) -> Self {
        let current_index: usize = self as usize;
        let previous_index = current_index.saturating_sub(1);

        if current_index != previous_index {
            state.list_state.select(Some(0));
        }

        Self::from_repr(previous_index).unwrap_or(self)
    }

    // Get the next tab, if there is no next tab return the current tab.
    pub fn next(self, state: &mut GeneralState) -> Self {
        let current_index = self as usize;
        let next_index = current_index.saturating_add(1);

        // Check if the next index is out of bounds
        Self::from_repr(next_index).map(|_| {
            // If the next index is valid, select the first item in the list
            state.list_state.select(Some(0));
        });

        Self::from_repr(next_index).unwrap_or(self)
    }

    pub fn render_radio(self, area: Rect, buf: &mut Buffer) {
        Paragraph::new("Radio")
            .block(self._container())
            .render(area, buf);
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
