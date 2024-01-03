use std::collections::HashMap;

use crate::{
    game_mode::GameMode,
    widgets::card::{self, Card, CardState, CardWidgetState},
};
use once_cell::sync::Lazy;
use rand::{seq::SliceRandom, thread_rng};

pub struct GameState {
    pub mode: GameMode,
    pub board: HashMap<(u8, u8), Card>,
    row_count: u8,
    column_count: u8,
    row: u8,
    column: u8,
    opened_card_count: u8,
    number_of_moves: u16,
    pub finished: bool,
    pub selected_symbol: CardWidgetState,
}
static CARD_CHARS: Lazy<Vec<char>> = Lazy::new(|| {
    Vec::from([
        'A', 'B', 'C', 'D', 'E', 'F', 'G', 'H', 'I', 'J', 'K', 'L', 'M', 'N', 'O', 'P', 'Q', 'R',
        'S', 'T', 'U', 'V', 'W', 'X', 'Y', 'Z',
    ])
});

impl GameState {
    pub fn new(mode: GameMode) -> Self {
        let board = Self::init_board(&mode);
        let mut row_count: u8 = 0;
        let mut column_count: u8 = 0;

        for kv in board.keys().into_iter() {
            if kv.0 > row_count {
                row_count = kv.0;
            }

            if kv.1 > column_count {
                column_count = kv.1;
            }
        }

        Self {
            mode,
            board,
            row_count,
            column_count,
            row: 0,
            column: 0,
            opened_card_count: 0,
            number_of_moves: 0,
            finished: false,
            selected_symbol: CardWidgetState {
                selected_id: (0, 0),
            },
        }
    }

    pub fn move_horizontal(&mut self, value: i8) {
        match value {
            -1 => {
                if self.column != 0 {
                    self.column -= 1;
                }
            }
            1 => {
                if self.column != self.column_count {
                    self.column += 1;
                }
            }
            _ => panic!("Invalid value. Value must be -1 or 1."),
        }
        self.selected_symbol.selected_id = (self.row, self.column);
    }

    pub fn move_vertical(&mut self, value: i8) {
        match value {
            -1 => {
                if self.row != 0 {
                    self.row -= 1;
                }
            }
            1 => {
                if self.row != self.row_count {
                    self.row += 1;
                }
            }
            _ => panic!("Invalid value. Value must be -1 or 1."),
        }
        self.selected_symbol.selected_id = (self.row, self.column);
    }

    pub fn flip(&mut self) {
        if self.opened_card_count == 2 {
            for card in self
                .board
                .values_mut()
                .filter(|x| x.state == CardState::OPENED)
            {
                card.state = CardState::CLOSED;
                self.opened_card_count = 0;
            }
            return;
        }

        let card = self.board.get_mut(&(self.row, self.column)).unwrap();
        if card.state == CardState::CLOSED {
            card.state = CardState::OPENED;
            self.opened_card_count += 1;
        } else {
            return;
        }
        //for entry in self.board.iter_mut() {
        //    let selected = self.row == entry.0 .0 && self.column == entry.0 .1;
        //    entry.1.set_selected(selected);
        //}

        let mut opened_cards: Vec<&mut Card> = self
            .board
            .values_mut()
            .filter(|x| x.state == CardState::OPENED)
            .collect();

        if opened_cards.len() == 2 {
            let resolved = Self::check(*opened_cards[0], *opened_cards[1]);
            if resolved {
                for card in opened_cards.iter_mut() {
                    card.state = CardState::RESOLVED;
                }
                self.finished = self.is_finish();
                self.opened_card_count = 0;
            }
        }
    }

    fn check(card_1: Card, card_2: Card) -> bool {
        card_1.symbol == card_2.symbol
    }

    fn is_finish(&self) -> bool {        
        !self.board.values().any(|x| x.state != CardState::RESOLVED)
    }

    fn init_board(mode: &GameMode) -> HashMap<(u8, u8), Card> {
        let number_of_pairs = mode.get_number_of_pairs();
        let capacity = number_of_pairs * 2;
        let mut result = HashMap::<(u8, u8), Card>::with_capacity(capacity.into());

        let (row, column) = mode.get_number_of_rows_and_columns();

        let mut card_chars: Vec<&char> = CARD_CHARS.iter().take(number_of_pairs.into()).collect();
        card_chars.append(&mut card_chars.clone());
        card_chars.shuffle(&mut thread_rng());
        let mut counter = 0;
        for r in 0..row {
            for c in 0..column {
                result.insert((r, c), Card::new((r, c), *card_chars[counter]));
                counter += 1;
            }
        }
        result
    }
}
