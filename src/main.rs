mod Tiles;
use Tiles::TileError;

use crate::Tiles::{Tile, TileState};
use std::io::{stdin, stdout, Write};

#[derive(Debug)]
struct Game {
    board: [Tile; 9],
    players: (String, String),
    next: Option<u8>,
}

static BOARD_WIDTH: usize = 9 * 3;
impl Game {
    pub fn new(tiles: [Tile; 9], players: (String, String)) -> Self {
        Game {
            board: tiles,
            players,
            next: None,
        }
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let mut row_counter = 0;
        let column_width = 7;
        writeln!(buffer, "").unwrap();
        writeln!(buffer, "--{:-^BOARD_WIDTH$}--", "").unwrap();
        for tile in &self.board {
            let state = match tile.get_state() {
                TileState::Empty => "Empty",
                TileState::X => "X",
                TileState::O => "O",
            };

            write!(
                buffer,
                "| {:^column_width$} ",
                if state == "Empty" {
                    tile.get_key()
                } else {
                    state
                }
            )
            .unwrap();

            if row_counter > 1 {
                write!(buffer, "|\n").unwrap();
                writeln!(buffer, "--{:-^BOARD_WIDTH$}--", "").unwrap();
                row_counter = 0;
                continue;
            }
            row_counter += 1;
        }
    }

    pub fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }

    pub fn validate(&self) -> bool {
        return false;
    }

    pub fn get_current_player(&self) -> &String {
        if self.next.is_none() || self.next.unwrap() == 0 {
            &self.players.0
        } else {
            &self.players.1
        }
    }

    pub fn next(&mut self) {
        let next_player = match self.next {
            None => 0,
            Some(0) => 1,
            Some(1) => 0,
            Some(_) => 0,
        };

        self.next = Some(next_player);
    }

    pub fn select_tile(&mut self, key: &str) -> Result<bool, &str> {
        let position = self.board.iter().position(|t| t.get_key() == key);
        match position {
            Some(pos) => {
                let player = self.next.unwrap();
                let state = if player == 0 {
                    TileState::X
                } else {
                    TileState::O
                };
                match self.board[pos].set_state(state) {
                    Ok(_) => Ok(true),
                    Err(_) => Err("This tile is already in use"),
                }
            }
            None => Err("Tile is out of bounds"),
        }
    }
}

fn start_game() {
    let tile_keys = ["a1", "a2", "a3", "b1", "b2", "b3", "c1", "c2", "c3"];
    let tiles = tile_keys.map(|key| Tile::new(key.to_string()));

    let players = (String::from("Player1"), String::from("Player2"));
    let mut game = Game::new(tiles, players);

    let mut p1 = String::new();
    let mut p2 = String::new();
    println!("Enter player 1 name...");
    let _ = stdout().flush();
    stdin().read_line(&mut p1).unwrap();

    println!("Enter player 2 name...");
    let _ = stdout().flush();
    stdin().read_line(&mut p2).unwrap();

    println!("Lets start the game! {} goes first.", p1);

    game.next();
    let mut complete = false;
    while !complete {
        let valid = game.validate();
        if valid {
            complete = true;
        } else {
            game.draw();

            let _ = stdout().flush();
            let mut tile = String::new();

            println!("It's {}'s turn...", game.get_current_player());
            stdin().read_line(&mut tile).unwrap();

            match game.select_tile(&tile.trim()) {
                Ok(_) => game.next(),
                Err(_) => println!("Tile {tile} is out of bounds"),
            }
        }
    }
}

fn main() {
    start_game();
}
