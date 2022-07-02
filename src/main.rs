pub mod core;
mod terminal;
pub mod utils;

fn main() {
    let mut app = terminal::Terminal::new();
    app.run_terminal();
}
