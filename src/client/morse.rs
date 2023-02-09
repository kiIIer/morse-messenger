use crate::morser::Signal;
use futures::FutureExt;
use futures_core::Stream;
use futures_timer::Delay;
use std::time::{Duration, Instant};
use tokio::select;
use tokio::sync::mpsc::{unbounded_channel, UnboundedReceiver, UnboundedSender};
use tokio_stream::StreamExt;
use tonic::Streaming;
use Morse::{Dah, Dit, Space};

#[derive(Debug, PartialEq)]
pub enum Morse {
    Dit,
    Dah,
    Space,
    None,
}

#[derive(Debug, PartialEq)]
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
    Dot,
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

impl From<Vec<Morse>> for Letter {
    fn from(value: Vec<Morse>) -> Self {
        if value.len() == 0 || value.len() > 6 {
            return Letter::None;
        }

        if compare(&value, &vec![Dit, Dah]) {
            return Letter::A;
        }
        if compare(&value, &vec![Dah, Dit, Dit, Dit]) {
            return Letter::B;
        }
        if compare(&value, &vec![Dah, Dit, Dah, Dit]) {
            return Letter::C;
        }
        if compare(&value, &vec![Dah, Dit, Dit]) {
            return Letter::D;
        }
        if compare(&value, &vec![Dit]) {
            return Letter::E;
        }
        if compare(&value, &vec![Dit, Dit, Dah, Dit]) {
            return Letter::F;
        }
        if compare(&value, &vec![Dah, Dah, Dit]) {
            return Letter::G;
        }
        if compare(&value, &vec![Dit, Dit, Dit, Dit]) {
            return Letter::H;
        }
        if compare(&value, &vec![Dit, Dit]) {
            return Letter::I;
        }
        if compare(&value, &vec![Dit, Dah, Dah, Dah]) {
            return Letter::J;
        }
        if compare(&value, &vec![Dah, Dit, Dah]) {
            return Letter::K;
        }
        if compare(&value, &vec![Dit, Dah, Dit, Dit]) {
            return Letter::L;
        }
        if compare(&value, &vec![Dah, Dah]) {
            return Letter::M;
        }
        if compare(&value, &vec![Dah, Dit]) {
            return Letter::N;
        }
        if compare(&value, &vec![Dah, Dah, Dah]) {
            return Letter::O;
        }
        if compare(&value, &vec![Dit, Dah, Dah, Dit]) {
            return Letter::P;
        }
        if compare(&value, &vec![Dah, Dah, Dit, Dah]) {
            return Letter::Q;
        }
        if compare(&value, &vec![Dit, Dah, Dit]) {
            return Letter::R;
        }
        if compare(&value, &vec![Dit, Dit, Dit]) {
            return Letter::S;
        }
        if compare(&value, &vec![Dah]) {
            return Letter::T;
        }
        if compare(&value, &vec![Dit, Dit, Dah]) {
            return Letter::U;
        }
        if compare(&value, &vec![Dit, Dit, Dit, Dah]) {
            return Letter::V;
        }
        if compare(&value, &vec![Dit, Dah, Dah]) {
            return Letter::W;
        }
        if compare(&value, &vec![Dah, Dit, Dit, Dah]) {
            return Letter::X;
        }
        if compare(&value, &vec![Dah, Dit, Dah, Dah]) {
            return Letter::Y;
        }
        if compare(&value, &vec![Dah, Dah, Dit, Dit]) {
            return Letter::Z;
        }
        if compare(&value, &vec![Dah, Dah, Dah, Dah, Dah]) {
            return Letter::N0;
        }
        if compare(&value, &vec![Dit, Dah, Dah, Dah, Dah]) {
            return Letter::N1;
        }
        if compare(&value, &vec![Dit, Dit, Dah, Dah, Dah]) {
            return Letter::N2;
        }
        if compare(&value, &vec![Dit, Dit, Dit, Dah, Dah]) {
            return Letter::N3;
        }
        if compare(&value, &vec![Dit, Dit, Dit, Dit, Dah]) {
            return Letter::N4;
        }
        if compare(&value, &vec![Dit, Dit, Dit, Dit, Dit]) {
            return Letter::N5;
        }
        if compare(&value, &vec![Dah, Dit, Dit, Dit, Dit]) {
            return Letter::N6;
        }
        if compare(&value, &vec![Dah, Dah, Dit, Dit, Dit]) {
            return Letter::N7;
        }
        if compare(&value, &vec![Dah, Dah, Dah, Dit, Dit]) {
            return Letter::N8;
        }
        if compare(&value, &vec![Dah, Dah, Dah, Dah, Dit]) {
            return Letter::N9;
        }
        if compare(&value, &vec![Dit, Dah, Dit, Dah, Dit, Dah]) {
            return Letter::Dot;
        }

        Letter::None
    }
}

fn compare(v1: &Vec<Morse>, v2: &Vec<Morse>) -> bool {
    if v1.len() != v2.len() {
        return false;
    }

    for i in 0..v1.len() {
        if v1[i] != v2[i] {
            return false;
        }
    }

    true
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
            Letter::Dot => vec![Dit, Dah, Dit, Dah, Dit, Dah],
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
            Letter::Dot => '.',
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
    tx_counter: UnboundedSender<()>,
    time_unit: Duration,
) {
    while let Some(letter) = rx.recv().await {
        let morse_v: Vec<Morse> = (&letter).into();
        for morse in morse_v {
            morse_transmitter(tx.clone(), time_unit, morse).await;
        }

        Delay::new(time_unit * 2).fuse().await;

        tx_counter.send(()).expect("Couldn't count");
    }
}

#[derive(Debug)]
enum MorseDelay {
    Letter,
    Word,
}

#[derive(Debug)]
enum MorseSignal {
    Off(MorseDelay),
    On(Morse),
}

async fn signal_receiver(
    mut signal_in: UnboundedReceiver<Signal>,
    tx_signal: UnboundedSender<(bool, Instant)>,
) {
    while let Some(signal) = signal_in.recv().await {
        tx_signal
            .send((signal.state, Instant::now()))
            .expect("This cannot not fail");
    }
}

async fn morse_receiver(
    signal_in: UnboundedReceiver<Signal>,
    tx_morse: UnboundedSender<MorseSignal>,
    time_unit: Duration,
    precision: f64,
) {
    let (tx_s, mut rx_s) = unbounded_channel();

    tokio::spawn(signal_receiver(signal_in, tx_s));

    let mut last = (false, Instant::now());
    loop {
        let delay = Delay::new(time_unit * 7).fuse();

        select! {
            _ = delay => {
                tx_morse.send(MorseSignal::Off(MorseDelay::Word)).expect("Will not fail");
            }

            state = rx_s.recv() => {
                if let Some(state) = state{

                       let duration = last.1.elapsed();
        let current = state.0;
        let now = state.1;

        if last.0 == current {
            continue;
        }

        last = (current, now);

        // TODO: rename
        let ratio = if duration > time_unit * 7 {
            7.0
        } else {
            duration.as_millis() as f64 / time_unit.as_millis() as f64
        };

        // TODO: rewrite to math like format
        if ratio < (1.0 + precision) && ratio > (1.0 - precision) {
            if current {
                tx_morse
                    .send(MorseSignal::Off(MorseDelay::Letter))
                    .expect("Wtf happened?");
                continue;
            }
            {
                tx_morse
                    .send(MorseSignal::On(Morse::Dit))
                    .expect("This will work");
                continue;
            }
        }

        let ratio = ratio / 3.0;

        if ratio < (1.0 + precision) && ratio > (1.0 - precision) {
            if current {
                tx_morse
                    .send(MorseSignal::Off(MorseDelay::Word))
                    .expect("This will");
                continue;
            } else {
                tx_morse.send(MorseSignal::On(Morse::Dah)).expect("WORK");
                continue;
            }
        }

        let ratio = ratio / 2.33333;

        if ratio < (1.0 + precision) && ratio > (1.0 - precision) {
            if current {
                tx_morse
                    .send(MorseSignal::Off(MorseDelay::Word))
                    .expect("Fail");
                tx_morse.send(MorseSignal::On(Morse::Space)).expect("Smth");

                continue;
            }
        }

        tx_morse.send(MorseSignal::On(Morse::None)).expect("Idk");

                }
            }
        }
    }
}

pub async fn letter_receiver(
    signal_in: UnboundedReceiver<Signal>,
    tx_l: UnboundedSender<Letter>,
    time_unit: Duration,
    precision: f64,
) {
    let mut buffer = Vec::new();

    let (tx, mut rx) = unbounded_channel();
    tokio::spawn(morse_receiver(signal_in, tx, time_unit, precision));

    while let Some(morse_signal) = rx.recv().await {
        match morse_signal {
            MorseSignal::Off(delay) => match delay {
                MorseDelay::Letter => {}
                MorseDelay::Word => {
                    if buffer.is_empty() {
                        continue;
                    }
                    let try_letter: Vec<Morse> = buffer.drain(0..buffer.len()).collect();
                    let letter = Letter::from(try_letter);
                    tx_l.send(letter).expect("Couldn't send letter");
                }
            },
            MorseSignal::On(morse) => match morse {
                Morse::None => {
                    buffer.clear();
                    tx_l.send(Letter::None).expect("Couldn't send letter");
                }
                Dah => {
                    buffer.push(Dah);
                }
                Dit => {
                    buffer.push(Dit);
                }
                Space => {
                    tx_l.send(Letter::Space).expect("Couldn't send letter");
                }
            },
        }
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
        '.' => Some(Letter::Dot),
        ' ' => Some(Letter::Space),
        _ => None,
    }
}
