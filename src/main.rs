use std::io::Result;

use crossterm::{terminal::{enable_raw_mode, EnterAlternateScreen, LeaveAlternateScreen, disable_raw_mode}, event::{EnableMouseCapture, self, KeyEventKind, KeyCode, DisableMouseCapture}, execute};
use ratatui::{TerminalOptions, Viewport, Terminal, backend::CrosstermBackend, widgets::Paragraph, style::Stylize};

fn main() -> Result<()> {
    let options = TerminalOptions {
        viewport: Viewport::Fullscreen,
    };
    let mut terminal = Terminal::with_options(CrosstermBackend::new(std::io::stdout()), options)?;
    enable_raw_mode()?;
    execute!(std::io::stdout(), EnterAlternateScreen, EnableMouseCapture)?;

    loop {
        terminal.draw(|frame| {
            let area = frame.size();
            frame.render_widget(
                Paragraph::new("Memory game! (press 'q' to quit)")
                    .white()
                    .on_blue(),
                area,
            );
        })?;

        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press
                    && key.code == KeyCode::Char('q')
                {
                    break;
                }
            }
        }
    }

    execute!(std::io::stdout(), LeaveAlternateScreen, DisableMouseCapture)?;
    disable_raw_mode()?;
    Ok(())
}
