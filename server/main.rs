mod tui;

fn main() {
    tui::tui_main().map_err(|e| anyhow::anyhow!("TUI error: {}", e));
}
