use std::io::Result;
use crossterm::event::{self, KeyEventKind, KeyCode};

use crate::{game_state::GameState, tui::Tui, game_mode::GameMode, ui::render};

pub struct App {
    tui: Tui,
    running: bool,
    pub state: GameState
}

impl App {
    pub fn new(mode: GameMode) -> Result<Self> {
        Ok(Self {
            tui: Tui::start()?,
            running: true,
            state: GameState::new(mode)
        })
    }


    pub fn run(mode: GameMode) -> Result<()> {
        install_panic_hook();
        let mut app = Self::new(mode)?;
        while app.running {
            app.draw()?;
            app.handle_events()?;
        }
        Tui::stop()?;
        Ok(())
    }

    fn draw(&mut self) -> Result<()> {
        self.tui.terminal
            .draw(|frame| render(frame, &mut self.state ))?;
        Ok(())
    }

    fn handle_events(&mut self) -> Result<()> {
        if event::poll(std::time::Duration::from_millis(16))? {
            if let event::Event::Key(key) = event::read()? {
                if key.kind == KeyEventKind::Press {
                    if key.code == KeyCode::Char('q') {
                        self.running = false;
                    } else if key.code == KeyCode::Left {
                        self.state.move_horizontal(-1);
                    } else if key.code == KeyCode::Right {
                        self.state.move_horizontal(1);
                    } else if key.code == KeyCode::Up {
                        self.state.move_vertical(-1);
                    } else if key.code == KeyCode::Down {
                        self.state.move_vertical(1);
                    } else if key.code == KeyCode::Enter || key.code == KeyCode::Char(' ') {
                        if self.state.finished {
                            self.state = GameState::new(self.state.mode);
                        } else {
                            self.state.flip();
                        }
                    }
                }
            }
        }
        Ok(())
    }
}

pub fn install_panic_hook() {
    better_panic::install();
    let hook = std::panic::take_hook();
    std::panic::set_hook(Box::new(move |info| {
        let _ = Tui::stop();
        hook(info);
        std::process::exit(1);
    }));
}