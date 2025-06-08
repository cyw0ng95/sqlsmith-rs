use std::io;

mod app;
use crate::tui::app::App;

pub fn tui_main() -> io::Result<()> {
    let mut terminal = ratatui::init();
    let app_result = App::default().run(&mut terminal);
    ratatui::restore();
    app_result
}