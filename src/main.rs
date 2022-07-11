pub mod common;
pub mod core;
mod terminal;

fn main() {
    let mut app = terminal::Terminal::new();
    app.run_terminal();
}
