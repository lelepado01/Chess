
use amethyst::{
    ecs::{Component, DenseVecStorage}
};

use super::piece_types::{Bishop};

impl Component for Bishop {
    type Storage = DenseVecStorage<Self>; 
}

impl Bishop {
    pub fn move_is_available(&self, x : u8, y : u8) -> bool {
        if self.board_coords_x == x && self.board_coords_y == y {
            false
        } else if (self.board_coords_x as f32 - x as f32).abs() == (self.board_coords_y as f32 - y as f32).abs() {
            true
        } else {
            false
        }
    }
}