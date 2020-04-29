
use amethyst::ecs::{Component, DenseVecStorage};

use super::piece_types::{BlackPawn, WhitePawn};

impl Component for WhitePawn {
    type Storage = DenseVecStorage<Self>;
}

impl Component for BlackPawn {
    type Storage = DenseVecStorage<Self>;
}

impl WhitePawn {
    pub fn move_is_available(&self, x : u8, y : u8) -> bool {
        if self.board_coords_y == 0 {
            false
        } else if !self.moved && self.board_coords_y + 2 == y {
            true 
        } else if self.board_coords_x == x && self.board_coords_y + 1 == y {
            true
        } else {
            false
        }
    }
}

impl BlackPawn {
    pub fn move_is_available(&self, x : u8, y : u8) -> bool {
        if self.board_coords_y == 0 {
            false
        } else if !self.moved && self.board_coords_y - 2 == y {
            true 
        } else if self.board_coords_x == x && self.board_coords_y - 1 == y {
            true
        } else {
            false
        }
    }
}