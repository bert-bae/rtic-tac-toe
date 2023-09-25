mod Tiles;
use crate::Tiles::{Tile, TileState};

#[derive(Debug)]
struct Game {
    board: [Tile; 9],
    players: [String; 2],
}

static BOARD_WIDTH: usize = 9 * 3;
impl Game {
    pub fn new(tiles: [Tile; 9], players: [String; 2]) -> Self {
        Game {
            board: tiles,
            players,
        }
    }

    fn draw_into(&self, buffer: &mut dyn std::fmt::Write) {
        let mut row_counter = 0;

        writeln!(buffer, "-{:-^BOARD_WIDTH$}-", "").unwrap();
        for tile in &self.board {
            write!(buffer, "| {: ^BOARD_WIDTH$} ", tile.get_owner());
            if (row_counter > 1) {
                write!(buffer, "|\n").unwrap();
                writeln!(buffer, "-{:-^BOARD_WIDTH$}-", "").unwrap();
                row_counter = 0;
                continue;
            }
            row_counter += 1;
        }
        // writeln!(buffer, "+-{:-<BOARD_WIDTH$}-+", "").unwrap();
        // writeln!(buffer, "| {:^BOARD_WIDTH$} |", "A1").unwrap();
        // writeln!(buffer, "+-{:-<BOARD_WIDTH$}-+", "").unwrap();
    }

    pub fn draw(&self) {
        let mut buffer = String::new();
        self.draw_into(&mut buffer);
        println!("{buffer}");
    }
}

fn main() {
    let tile_keys = ["a1", "a2", "a3", "b1", "b2", "b3", "c1", "c2", "c3"];
    let mut tiles = tile_keys.map(|key| Tile::new(key.to_string()));
    // tiles[0].set_owner(TileState::X);
    // println!("{tiles:?}");

    let players = [String::from("Player1"), String::from("Player2")];
    let game = Game::new(tiles, players);
    // println!("{game:?}");
    game.draw();
}
