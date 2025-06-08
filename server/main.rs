mod tui;

fn main() {
    sqlsmith_rs_common::logger::init(); // Configure logging

    tui::tui_main();
}
