use std::fmt;

pub enum GameStatus {
    Over,
    Tie,
    Play,
}

impl fmt::Display for GameStatus {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            GameStatus::Over => write!(f, "Over"),
            GameStatus::Tie => write!(f, "Tie"),
            GameStatus::Play => write!(f, "Play"),
        }
    }
}
