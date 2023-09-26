mod tiles;

use crate::tiles::{ Tile, TileState };
use std::io::{ stdin, stdout, Write };

#[derive(Debug)]
struct Game {
    board: [[Tile; 3]; 3],
    players: (String, String),
    next: Option<u8>,
}

enum GameResult {
    Draw,
    Win,
}

static WIN_CONDITIONS: [[(usize, usize); 3]; 8] = [
    [
        (0, 0),
        (0, 1),
        (0, 2),
    ],
    [
        (1, 0),
        (1, 1),
        (1, 2),
    ],
    [
        (2, 0),
        (2, 1),
        (2, 2),
    ],
    [
        (0, 0),
        (1, 0),
        (2, 0),
    ],
    [
        (0, 1),
        (1, 1),
        (2, 1),
    ],
    [
        (0, 2),
        (1, 2),
        (2, 2),
    ],
    [
        (0, 0),
        (1, 1),
        (2, 2),
    ],
    [
        (0, 2),
        (1, 1),
        (2, 0),
    ],
];

static BOARD_WIDTH: usize = 9 * 3;
impl Game {
    fn new(tiles: [[Tile; 3]; 3], players: (String, String)) -> Self {
        Game {
            board: tiles,
            players,
            next: None,
        }
    }

    fn start(&mut self) {
        self.next();
        let current_player = self.get_current_player().clone();

        println!("Lets start the game! {} goes first.", current_player);
        let mut complete: Option<GameResult> = None;
        while complete.is_none() {
            self.draw();

            let _ = stdout().flush();
            let mut tile = String::new();

            println!("It's {}'s turn...", current_player.trim());
            stdin().read_line(&mut tile).unwrap();

            match self.select_tile(&tile.trim()) {
                Ok(_) => {
                    match self.validate() {
                        Some(x) => {
                            complete = Some(x);
                        }
                        None => (),
                    }
                    self.next();
                }
                Err(e) => println!("Invalid selection: {e}"),
            }
        }

        match complete {
            Some(GameResult::Draw) => println!("It's a draw."),
            Some(GameResult::Win) => println!("Player {current_player} is the winner!"),
            None => (),
        }
    }
    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let column_width = 7;
        writeln!(buffer, "").unwrap();
        for row in &self.board {
            writeln!(buffer, "--{:-^BOARD_WIDTH$}--", "").unwrap();
            for tile in row {
                let state = match tile.get_state() {
                    TileState::Empty => "Empty",
                    TileState::X => "X",
                    TileState::O => "O",
                };

                write!(buffer, "| {:^column_width$} ", if state == "Empty" {
                    tile.get_key()
                } else {
                    state
                }).unwrap();
            }
            write!(buffer, "|").unwrap();
            writeln!(buffer, "").unwrap();
        }
        writeln!(buffer, "--{:-^BOARD_WIDTH$}--", "").unwrap();
    }

    fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }

    fn validate(&self) -> Option<GameResult> {
        let mut available_tiles = false;
        let sym = if self.next.unwrap() == 0 { TileState::X } else { TileState::O };
        for condition in WIN_CONDITIONS {
            let winner = condition.iter().all(|(x, y)| {
                let tile = &self.board[*x][*y] as &Tile;
                if tile.get_state() == &TileState::Empty {
                    available_tiles = true;
                    return false;
                }

                return tile.get_state() == &sym;
            });
            if winner {
                return Some(GameResult::Win);
            }
        }

        if available_tiles {
            return None;
        }

        return Some(GameResult::Draw);
    }

    fn get_current_player(&self) -> &String {
        if self.next.is_none() || self.next.unwrap() == 0 {
            &self.players.0
        } else {
            &self.players.1
        }
    }

    fn next(&mut self) {
        let next_player = match self.next {
            None => 0,
            Some(0) => 1,
            Some(1) => 0,
            Some(_) => 0,
        };
        self.next = Some(next_player);
    }

    fn select_tile(&mut self, key: &str) -> Result<bool, String> {
        for row in self.board.iter_mut() {
            let position = row.iter().position(|t| t.get_key() == key);
            if position.is_some() {
                let player = self.next.unwrap();
                println!("player  {player:?}");
                let state = if player == 0 { TileState::X } else { TileState::O };

                if position.is_some() {
                    match row[position.unwrap()].set_state(state) {
                        Ok(_) => {
                            return Ok(true);
                        }
                        Err(e) => {
                            return Err(e.to_string());
                        }
                    }
                }
            }
        }
        Err(String::from("Out of bounds"))
    }
}

fn main() {
    let tiles = [
        [Tile::new("a1".to_string()), Tile::new("a2".to_string()), Tile::new("a3".to_string())],
        [Tile::new("b1".to_string()), Tile::new("b2".to_string()), Tile::new("b3".to_string())],
        [Tile::new("c1".to_string()), Tile::new("c2".to_string()), Tile::new("c3".to_string())],
    ];

    let mut p1 = String::new();
    let mut p2 = String::new();
    println!("Enter player 1 name...");
    let _ = stdout().flush();
    stdin().read_line(&mut p1).unwrap();

    println!("Enter player 2 name...");
    let _ = stdout().flush();
    stdin().read_line(&mut p2).unwrap();

    let mut game = Game::new(tiles, (p1, p2));
    game.start();
}
