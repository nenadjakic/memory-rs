use ratatui::{widgets::{Widget, Block, Borders, BorderType, Paragraph, StatefulWidget}, style::Color, layout::Alignment};

use crate::utils::render_ascii_char;

#[derive(Clone, Copy)]
pub enum CardState {
    OPENED,
    CLOSED,
    RESOLVED,
}

pub struct CardWidgetState {
    pub selected_id: (u8, u8)
}

#[derive(Clone, Copy)]
pub struct Card {
    pub id: (u8, u8),
    pub symbol: char,
    selected: bool,
    state: CardState,
}

impl Card {
    pub fn new(id: (u8, u8), symbol: char) -> Self {
        Self {
            id,
            symbol,
            state: CardState::CLOSED,
            selected: false
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

impl StatefulWidget for Card {
    type State = CardWidgetState;

    fn render(self, area: ratatui::prelude::Rect, buf: &mut ratatui::prelude::Buffer, state: &mut Self::State) {
        let box_color = Color::LightYellow;

        let block = Block::default()
            .borders(Borders::ALL)
            .border_type(if state.selected_id == self.id { BorderType::Thick } else { BorderType::Plain })
            .border_style(ratatui::style::Style::default().fg(box_color));

        let text_case_widget = Paragraph::new(render_ascii_char(self.symbol.to_ascii_lowercase()))
            .block(block.clone())
            .alignment(Alignment::Center)
            .style(ratatui::style::Style::default().fg(Color::LightRed));

        text_case_widget.clone().render(area, buf);
        block.clone().render(area, buf);
    }
}