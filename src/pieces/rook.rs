
use amethyst::{
    ecs::{Component, DenseVecStorage}

};

use super::piece_types::{Rook};

impl Component for Rook {
    type Storage = DenseVecStorage<Self>; 
}

impl Rook {
    pub fn move_is_available(&self, x : u8, y : u8) -> bool {
        !(self.board_coords_x == x && self.board_coords_y == y) 
        && 
        (self.board_coords_x == x || self.board_coords_y == y)
    }
}
