use crate::ascii_constants::{
    ASCII_A, ASCII_B, ASCII_C, ASCII_D, ASCII_E, ASCII_EMPTY, ASCII_F, ASCII_G, ASCII_H, ASCII_I,
    ASCII_J, ASCII_K, ASCII_L, ASCII_M, ASCII_N, ASCII_O, ASCII_P, ASCII_Q, ASCII_R, ASCII_S,
    ASCII_T, ASCII_U, ASCII_V, ASCII_W, ASCII_X, ASCII_Y, ASCII_Z,
};

pub fn render_ascii_char(c: char) -> String {
    let mut result: String = String::new();
    //String::from(c.to_ascii_uppercase())

    let ascii = match c {
        'a' => ASCII_A,
        'b' => ASCII_B,
        'c' => ASCII_C,
        'd' => ASCII_D,
        'e' => ASCII_E,
        'f' => ASCII_F,
        'g' => ASCII_G,
        'h' => ASCII_H,
        'i' => ASCII_I,
        'j' => ASCII_J,
        'k' => ASCII_K,
        'l' => ASCII_L,
        'm' => ASCII_M,
        'n' => ASCII_N,
        'o' => ASCII_O,
        'p' => ASCII_P,
        'q' => ASCII_Q,
        'r' => ASCII_R,
        's' => ASCII_S,
        't' => ASCII_T,
        'u' => ASCII_U,
        'v' => ASCII_V,
        'w' => ASCII_W,
        'x' => ASCII_X,
        'y' => ASCII_Y,
        'z' => ASCII_Z,
        _ => ASCII_EMPTY,
    };
    for &x in ascii.iter() {
        result += x;
        result += "\n";
    }
    result
}
