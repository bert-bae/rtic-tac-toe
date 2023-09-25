mod Tiles;
use crate::Tiles::{Tile, TileState};

struct Game {
    board: [Tile; 9],
    players: [String; 2],
}

fn main() {
    let tile_keys = ["a1", "a2", "a3", "b1", "b2", "b3", "c1", "c2", "c3"];
    let tiles = tile_keys.map(|key| Tile::new(key.to_string()));
    println!("{tiles:?}")
}
