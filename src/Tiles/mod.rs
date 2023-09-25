use derive_more::Display;
use thiserror::{self, Error};

#[derive(Debug, Error)]
pub enum TileError {
    #[error("Tile {0} is already in use.")]
    TileInUse(String),
}

#[derive(Debug, Display, PartialEq)]
pub enum TileState {
    Empty,
    X,
    Y,
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

    pub fn set_owner(&mut self, owner: TileState) -> Result<&Tile, TileError> {
        if self.state != TileState::Empty {
            return Err(TileError::TileInUse(self.key.clone()));
        }

        self.state = owner;
        return Ok(self);
    }

    pub fn get_owner(&self) -> &TileState {
        return &self.state;
    }
}
