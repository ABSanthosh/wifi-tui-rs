use color_eyre::eyre::Result;

mod list;
mod app;
mod ui;

fn main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();

    let app = app::App::new();
    let result = app.run(terminal);
    ratatui::restore();
    result
}
