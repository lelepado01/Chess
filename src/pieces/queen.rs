
use amethyst::{ 
    ecs::{Component, DenseVecStorage}

};

use super::piece_types::{Queen};

impl Component for Queen {
    type Storage = DenseVecStorage<Self>; 
}

impl Queen {
    pub fn move_is_available(&self, x : u8, y : u8) -> bool {
        !(self.board_coords_x == x && self.board_coords_y == y) 
        && 
        (
            self.board_coords_x == x 
            || 
            self.board_coords_y == y
            ||
            (self.board_coords_x as f32 - x as f32).abs() == (self.board_coords_y as f32 - y as f32).abs()
        )
    }
}