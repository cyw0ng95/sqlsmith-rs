mod app;
use crate::tui::app::App;
use color_eyre::Result;

pub fn tui_main() -> Result<()> {
    color_eyre::install()?;
    let terminal = ratatui::init();
    let app_result = App::default().run(terminal);
    ratatui::restore();
    app_result
}