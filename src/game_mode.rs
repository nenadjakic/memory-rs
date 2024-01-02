#[derive(Clone, Copy)]
pub enum GameMode {
    FOUR,
    EIGHT,
    TWELVE,
    SIXTEEN,
}

impl GameMode {
    pub fn get_number_of_pairs(self) -> u8 {
        match self {
            GameMode::FOUR => 4,
            GameMode::EIGHT => 8,
            GameMode::TWELVE => 12,
            GameMode::SIXTEEN => 16,
        }
    }

    pub fn get_number_of_rows_and_columns(self) -> (u8, u8) {
        match self.get_number_of_pairs() {
            4 => (2, 4),
            8 => (4, 4),
            12 => (6, 4),
            16 => (6, 6),
            _ => panic!("Illegal arguments."),
        }
    }
}
