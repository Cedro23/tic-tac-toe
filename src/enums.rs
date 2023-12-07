use std::fmt;
pub enum GameStatus {
    Play,
    Over,
    Tie,
}

impl fmt::Debug for GameStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{:?}", self)
    }
}
