pub enum Morse {
    Dit,
    Dah,
    Space,
}

impl From<Morse> for char {
    fn from(value: Morse) -> Self {
        match value {
            Dit => '•',
            Dah => '—',
            No => ' ',
        }
    }
}
