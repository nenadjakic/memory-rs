use std::collections::HashMap;

use crate::{game_mode::GameMode, widgets::card::Card};
use once_cell::sync::Lazy;
use rand::{seq::SliceRandom, thread_rng};

pub struct GameState {
    pub mode: GameMode,
    pub board: HashMap<(u8, u8), Card>,
    row_count: u8,
    column_count: u8,
    row: u8,
    column: u8
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
        }
    }

    pub fn move_horizontal(&self, value: i8) {}

    pub fn move_vertical(&self, value: i8) {}

    pub fn select_card(&mut self) {
        
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
                let selected = r == 0 && c == 0;
                result.insert((r, c), Card::new(*card_chars[counter], selected));
                counter += 1;
            }
        }
        result
    }
}
