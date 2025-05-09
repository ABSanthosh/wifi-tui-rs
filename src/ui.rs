use crate::app::App;
use ratatui::prelude::*;
use ratatui::widgets::{Block, Borders, List, ListItem, Paragraph};

pub fn draw_ui<B: Backend>(f: &mut Frame<B>, app: &App) {
    let size = f.size();

    let chunks = Layout::default()
        .direction(Direction::Vertical)
        .margin(2)
        .constraints([Constraint::Min(5), Constraint::Length(3)])
        .split(size);

    let items: Vec<ListItem> = app
        .networks
        .iter()
        .map(|net| {
            let status = if net.connected { "[x]" } else { "[ ]" };
            let secure = if net.secured { "🔒" } else { "" };
            ListItem::new(format!(
                "{} {} - {}% {}",
                status, net.ssid, net.signal, secure
            ))
        })
        .collect();

    let list = List::new(items)
        .block(
            Block::default()
                .title("Available Networks")
                .borders(Borders::ALL),
        )
        .highlight_style(Style::default().bg(Color::Blue).fg(Color::White))
        .highlight_symbol("▶ ");

    f.render_stateful_widget(
        list,
        chunks[0],
        &mut ListState::default().with_selected(Some(app.selected)),
    );

    let status = Paragraph::new(app.status.clone())
        .block(Block::default().title("Status").borders(Borders::ALL));

    f.render_widget(status, chunks[1]);
}
