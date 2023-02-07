use Morse::{Dah, Dit, Space};

pub enum Morse {
    Dit,
    Dah,
    Space,
}

pub enum Letters {
    A,
    B,
    C,
    D,
    E,
    F,
    G,
    H,
    I,
    J,
    K,
    L,
    M,
    N,
    O,
    P,
    Q,
    R,
    S,
    T,
    U,
    V,
    W,
    X,
    Y,
    Z,
    N0,
    N1,
    N2,
    N3,
    N4,
    N5,
    N6,
    N7,
    N8,
    N9,
    Space,
}

impl From<Morse> for char {
    fn from(value: Morse) -> Self {
        match value {
            Dit => '•',
            Dah => '—',
            Space => ' ',
        }
    }
}

impl From<Letters> for Vec<Morse> {
    fn from(value: Letters) -> Self {
        match value {
            Letters::A => vec![Dit, Dah],
            Letters::B => vec![Dah, Dit, Dit, Dit],
            Letters::C => vec![Dah, Dit, Dah, Dit],
            Letters::D => vec![Dah, Dit, Dit],
            Letters::E => vec![Dit],
            Letters::F => vec![Dit, Dit, Dah, Dit],
            Letters::G => vec![Dah, Dah, Dit],
            Letters::H => vec![Dit, Dit, Dit, Dit],
            Letters::I => vec![Dit, Dit],
            Letters::J => vec![Dit, Dah, Dah, Dah],
            Letters::K => vec![Dah, Dit, Dah],
            Letters::L => vec![Dit, Dah, Dit, Dit],
            Letters::M => vec![Dah, Dah],
            Letters::N => vec![Dah, Dit],
            Letters::O => vec![Dah, Dah, Dah],
            Letters::P => vec![Dit, Dah, Dah, Dit],
            Letters::Q => vec![Dah, Dah, Dit, Dah],
            Letters::R => vec![Dit, Dah, Dit],
            Letters::S => vec![Dit, Dit, Dit],
            Letters::T => vec![Dah],
            Letters::U => vec![Dit, Dit, Dah],
            Letters::V => vec![Dit, Dit, Dit, Dah],
            Letters::W => vec![Dit, Dah, Dah],
            Letters::X => vec![Dah, Dit, Dit, Dah],
            Letters::Y => vec![Dah, Dit, Dah, Dah],
            Letters::Z => vec![Dah, Dah, Dit, Dit],
            Letters::N0 => vec![Dah, Dah, Dah, Dah, Dah],
            Letters::N1 => vec![Dit, Dah, Dah, Dah, Dah],
            Letters::N2 => vec![Dit, Dit, Dah, Dah, Dah],
            Letters::N3 => vec![Dit, Dit, Dit, Dah, Dah],
            Letters::N4 => vec![Dit, Dit, Dit, Dit, Dah],
            Letters::N5 => vec![Dit, Dit, Dit, Dit, Dit],
            Letters::N6 => vec![Dah, Dit, Dit, Dit, Dit],
            Letters::N7 => vec![Dah, Dah, Dit, Dit, Dit],
            Letters::N8 => vec![Dah, Dah, Dah, Dit, Dit],
            Letters::N9 => vec![Dah, Dah, Dah, Dah, Dit],
            Letters::Space => vec![Space],
        }
    }
}

impl From<Letters> for char {
    fn from(value: Letters) -> Self {
        match value {
            Letters::A => 'A',
            Letters::B => 'B',
            Letters::C => 'C',
            Letters::D => 'D',
            Letters::E => 'E',
            Letters::F => 'F',
            Letters::G => 'G',
            Letters::H => 'H',
            Letters::I => 'I',
            Letters::J => 'J',
            Letters::K => 'K',
            Letters::L => 'L',
            Letters::M => 'M',
            Letters::N => 'N',
            Letters::O => 'O',
            Letters::P => 'P',
            Letters::Q => 'Q',
            Letters::R => 'R',
            Letters::S => 'S',
            Letters::T => 'T',
            Letters::U => 'U',
            Letters::V => 'V',
            Letters::W => 'W',
            Letters::X => 'X',
            Letters::Y => 'Y',
            Letters::Z => 'Z',
            Letters::N0 => '0',
            Letters::N1 => '1',
            Letters::N2 => '2',
            Letters::N3 => '3',
            Letters::N4 => '4',
            Letters::N5 => '5',
            Letters::N6 => '6',
            Letters::N7 => '7',
            Letters::N8 => '8',
            Letters::N9 => '9',
            Letters::Space => ' ',
        }
    }
}
