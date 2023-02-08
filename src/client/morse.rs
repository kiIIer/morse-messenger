use crate::morser::Signal;
use futures::FutureExt;
use futures_timer::Delay;
use std::time::Duration;
use tokio::sync::mpsc::{UnboundedReceiver, UnboundedSender};
use Morse::{Dah, Dit, Space};

#[derive(Debug)]
pub enum Morse {
    Dit,
    Dah,
    Space,
    None,
}

#[derive(Debug)]
pub enum Letter {
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
    None,
}

impl From<Morse> for char {
    fn from(value: Morse) -> Self {
        let l = &value;
        l.into()
    }
}

impl From<&Morse> for char {
    fn from(value: &Morse) -> Self {
        match value {
            Dit => '•',
            Dah => '—',
            Space => ' ',
            Morse::None => '?',
        }
    }
}

impl From<&Letter> for Vec<Morse> {
    fn from(value: &Letter) -> Self {
        match value {
            Letter::A => vec![Dit, Dah],
            Letter::B => vec![Dah, Dit, Dit, Dit],
            Letter::C => vec![Dah, Dit, Dah, Dit],
            Letter::D => vec![Dah, Dit, Dit],
            Letter::E => vec![Dit],
            Letter::F => vec![Dit, Dit, Dah, Dit],
            Letter::G => vec![Dah, Dah, Dit],
            Letter::H => vec![Dit, Dit, Dit, Dit],
            Letter::I => vec![Dit, Dit],
            Letter::J => vec![Dit, Dah, Dah, Dah],
            Letter::K => vec![Dah, Dit, Dah],
            Letter::L => vec![Dit, Dah, Dit, Dit],
            Letter::M => vec![Dah, Dah],
            Letter::N => vec![Dah, Dit],
            Letter::O => vec![Dah, Dah, Dah],
            Letter::P => vec![Dit, Dah, Dah, Dit],
            Letter::Q => vec![Dah, Dah, Dit, Dah],
            Letter::R => vec![Dit, Dah, Dit],
            Letter::S => vec![Dit, Dit, Dit],
            Letter::T => vec![Dah],
            Letter::U => vec![Dit, Dit, Dah],
            Letter::V => vec![Dit, Dit, Dit, Dah],
            Letter::W => vec![Dit, Dah, Dah],
            Letter::X => vec![Dah, Dit, Dit, Dah],
            Letter::Y => vec![Dah, Dit, Dah, Dah],
            Letter::Z => vec![Dah, Dah, Dit, Dit],
            Letter::N0 => vec![Dah, Dah, Dah, Dah, Dah],
            Letter::N1 => vec![Dit, Dah, Dah, Dah, Dah],
            Letter::N2 => vec![Dit, Dit, Dah, Dah, Dah],
            Letter::N3 => vec![Dit, Dit, Dit, Dah, Dah],
            Letter::N4 => vec![Dit, Dit, Dit, Dit, Dah],
            Letter::N5 => vec![Dit, Dit, Dit, Dit, Dit],
            Letter::N6 => vec![Dah, Dit, Dit, Dit, Dit],
            Letter::N7 => vec![Dah, Dah, Dit, Dit, Dit],
            Letter::N8 => vec![Dah, Dah, Dah, Dit, Dit],
            Letter::N9 => vec![Dah, Dah, Dah, Dah, Dit],
            Letter::Space => vec![Space],
            Letter::None => vec![Morse::None],
        }
    }
}

impl From<Letter> for char {
    fn from(value: Letter) -> Self {
        (&value).into()
    }
}

impl From<&Letter> for char {
    fn from(value: &Letter) -> Self {
        match value {
            Letter::A => 'A',
            Letter::B => 'B',
            Letter::C => 'C',
            Letter::D => 'D',
            Letter::E => 'E',
            Letter::F => 'F',
            Letter::G => 'G',
            Letter::H => 'H',
            Letter::I => 'I',
            Letter::J => 'J',
            Letter::K => 'K',
            Letter::L => 'L',
            Letter::M => 'M',
            Letter::N => 'N',
            Letter::O => 'O',
            Letter::P => 'P',
            Letter::Q => 'Q',
            Letter::R => 'R',
            Letter::S => 'S',
            Letter::T => 'T',
            Letter::U => 'U',
            Letter::V => 'V',
            Letter::W => 'W',
            Letter::X => 'X',
            Letter::Y => 'Y',
            Letter::Z => 'Z',
            Letter::N0 => '0',
            Letter::N1 => '1',
            Letter::N2 => '2',
            Letter::N3 => '3',
            Letter::N4 => '4',
            Letter::N5 => '5',
            Letter::N6 => '6',
            Letter::N7 => '7',
            Letter::N8 => '8',
            Letter::N9 => '9',
            Letter::Space => ' ',
            Letter::None => '?',
        }
    }
}

pub async fn morse_transmitter(tx: UnboundedSender<Signal>, time_unit: Duration, morse: Morse) {
    let time_unit = time_unit.clone();
    match morse {
        Dit => {
            tx.send(Signal { state: true })
                .expect("Couldn't send to server");

            Delay::new(time_unit).fuse().await;

            tx.send(Signal { state: false })
                .expect("Couldn't send to server");
        }

        Dah => {
            tx.send(Signal { state: true })
                .expect("Couldn't send to server");

            Delay::new(time_unit * 3).fuse().await;

            tx.send(Signal { state: false })
                .expect("Couldn't send to server");
        }

        Space => {
            Delay::new(time_unit).fuse().await;
        }
        Morse::None => {}
    };
    Delay::new(time_unit).fuse().await;
}

pub async fn letter_transmitter(
    mut rx: UnboundedReceiver<Letter>,
    tx: UnboundedSender<Signal>,
    time_unit: Duration,
) {
    while let Some(letter) = rx.recv().await {
        let morse_v: Vec<Morse> = (&letter).into();
        for morse in morse_v {
            morse_transmitter(tx.clone(), time_unit, morse).await;
        }

        Delay::new(time_unit * 3).fuse().await;
    }
}

pub fn convert(symbol: char) -> Option<Letter> {
    let symbol = symbol.to_lowercase().next()?;
    match symbol {
        'a' => Some(Letter::A),
        'b' => Some(Letter::B),
        'c' => Some(Letter::C),
        'd' => Some(Letter::D),
        'e' => Some(Letter::E),
        'f' => Some(Letter::F),
        'g' => Some(Letter::G),
        'h' => Some(Letter::H),
        'i' => Some(Letter::I),
        'j' => Some(Letter::J),
        'k' => Some(Letter::K),
        'l' => Some(Letter::L),
        'm' => Some(Letter::M),
        'n' => Some(Letter::N),
        'o' => Some(Letter::O),
        'p' => Some(Letter::P),
        'q' => Some(Letter::Q),
        'r' => Some(Letter::R),
        's' => Some(Letter::S),
        't' => Some(Letter::T),
        'u' => Some(Letter::U),
        'v' => Some(Letter::V),
        'w' => Some(Letter::W),
        'x' => Some(Letter::X),
        'y' => Some(Letter::Y),
        'z' => Some(Letter::Z),
        '0' => Some(Letter::N0),
        '1' => Some(Letter::N1),
        '2' => Some(Letter::N2),
        '3' => Some(Letter::N3),
        '4' => Some(Letter::N4),
        '5' => Some(Letter::N5),
        '6' => Some(Letter::N6),
        '7' => Some(Letter::N7),
        '8' => Some(Letter::N8),
        '9' => Some(Letter::N9),
        ' ' => Some(Letter::Space),
        _ => None,
    }
}
