
use amethyst::{
    ecs::{Component, DenseVecStorage}

};

use super::piece_types::{Knight};

impl Component for Knight {
    type Storage = DenseVecStorage<Self>; 
}

impl Knight {
    pub fn move_is_available(&self, x_b : u8, y_b : u8) -> bool {
        let x = x_b as i8;
        let y = y_b as i8;

        (self.board_coords_x as i8 +2 == x && self.board_coords_y as i8 +1 == y)
        ||
        (self.board_coords_x as i8 -2 == x && self.board_coords_y as i8 +1 == y)
        ||
        (self.board_coords_x as i8 +2 == x && self.board_coords_y as i8 -1 == y)
        ||
        (self.board_coords_x as i8 -2 == x && self.board_coords_y as i8 -1 == y)
        ||
        (self.board_coords_x as i8 +1 == x && self.board_coords_y as i8 +2 == y)
        ||
        (self.board_coords_x as i8 -1 == x && self.board_coords_y as i8 +2 == y)
        ||
        (self.board_coords_x as i8 +1 == x && self.board_coords_y as i8 -2 == y)
        ||
        (self.board_coords_x as i8 -1 == x && self.board_coords_y as i8 -2 == y)
    }
}

