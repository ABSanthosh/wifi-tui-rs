use crate::models::Network;

#[derive(Default)]
pub struct App {
    pub networks: Vec<Network>,
    pub selected: usize,
    pub status: String,
}

impl App {
    pub fn new() -> Self {
        Self {
            networks: vec![],
            selected: 0,
            status: String::from("Press 'r' to refresh."),
        }
    }

    pub fn next(&mut self) {
        if self.selected < self.networks.len().saturating_sub(1) {
            self.selected += 1;
        }
    }

    pub fn previous(&mut self) {
        if self.selected > 0 {
            self.selected -= 1;
        }
    }
}
