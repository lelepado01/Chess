
use amethyst::{
    ecs::{Component, DenseVecStorage}

};

use super::piece_types::{King};

impl Component for King {
    type Storage = DenseVecStorage<Self>; 
}

impl King {
    pub fn move_is_available(&self, x : u8, y : u8) -> bool {
        if self.board_coords_x == 0 && self.board_coords_y == 0 {
            self.board_coords_x + 1 == x || self.board_coords_y + 1 == y
        } else if self.board_coords_x == 0 {
            self.board_coords_x + 1 == x || self.board_coords_y + 1 == y || self.board_coords_y - 1 == y
        } else if self.board_coords_y == 0 {
            self.board_coords_x + 1 == x || self.board_coords_y + 1 == y || self.board_coords_x - 1 == x
        } else {
            self.board_coords_x + 1 == x || self.board_coords_x - 1 == x || self.board_coords_y + 1 == y || self.board_coords_y - 1 == y
        }
    }
}