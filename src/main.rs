use std::io::Result;

use app::App;

pub mod app;
pub mod ascii_constants;
pub mod game_mode;
pub mod game_state;
pub mod tui;
pub mod ui;
pub mod utils;
pub mod widgets;

fn main() -> Result<()> {
    App::run(game_mode::GameMode::EIGHT)?;

    Ok(())
}
