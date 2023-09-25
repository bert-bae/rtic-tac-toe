use thiserror::{self, Error};
use std::fmt;

#[derive(Debug, Error)]
pub enum TileError {
    #[error("Tile {0} is already in use.")]
    TileInUse(String),
}

#[derive(Debug, PartialEq)]
pub enum TileState {
    Empty,
    X,
    O,
}

impl fmt::Display for TileState {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            TileState::Empty => write!(f, "Empty"),
            TileState::X => write!(f, "X"),
            TileState::O => write!(f, "O"),
        }
    }
}

#[derive(Debug)]
pub struct Tile {
    state: TileState,
    key: String,
}

impl Tile {
    pub fn new(key: String) -> Self {
        Tile {
            state: TileState::Empty,
            key,
        }
    }

    pub fn set_state(&mut self, state: TileState) -> Result<&Tile, TileError> {
        if self.state != TileState::Empty {
            return Err(TileError::TileInUse(self.key.clone()));
        }

        self.state = state;
        return Ok(self);
    }

    pub fn get_state(&self) -> &TileState {
        return &self.state;
    }

    pub fn get_key(&self) -> &str {
        return &self.key;
    }
}
