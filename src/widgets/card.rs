use ratatui::{widgets::{Widget, Block, Borders, BorderType, Paragraph}, style::Color, layout::Alignment};

use crate::utils::render_ascii_char;

#[derive(Clone, Copy)]
pub enum CardState {
    OPENED,
    CLOSED,
    RESOLVED,
}

#[derive(Clone, Copy)]
pub struct Card {
    pub symbol: char,
    selected: bool,
    state: CardState,
}

impl Card {
    pub fn new(symbol: char, selected: bool) -> Self {
        Self {
            symbol,
            state: CardState::CLOSED,
            selected
        }
    }

    pub fn set_selected(&mut self, value: bool) {
        self.selected = value;
    }
}

impl Widget for Card {
    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer) {
        let box_color = Color::LightYellow;

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(if self.selected { BorderType::Thick } else { BorderType::Plain })
            .border_style(ratatui::style::Style::default().fg(box_color));

        let text_case_widget = Paragraph::new(render_ascii_char(self.symbol.to_ascii_lowercase()))
            .block(block.clone())
            .alignment(Alignment::Center)
            .style(ratatui::style::Style::default().fg(Color::LightRed));

        text_case_widget.clone().render(area, buf);
        block.clone().render(area, buf);
    }
}