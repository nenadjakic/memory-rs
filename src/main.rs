use std::io::Result;

use crossterm::event::{self, KeyCode, KeyEventKind};
use tui::Tui;
use ui::render;

pub mod ascii_constants;
pub mod tui;
pub mod ui;

fn main() -> Result<()> {
    let mut tui = Tui::start()?;

    loop {
        tui.terminal.draw(|frame| {
            render(frame);
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press && key.code == KeyCode::Char('q') {
                    break;
                }
            }
        }
    }

    let _ = Tui::stop();

    Ok(())
}
