
use amethyst::ecs::{Component, DenseVecStorage};

pub struct WhitePawn {
    pub board_coords_x : u8, 
    pub board_coords_y : u8, 
    pub selected : bool, 
    pub moved : bool, 
}

pub struct BlackPawn {
    pub board_coords_x : u8, 
    pub board_coords_y : u8, 
    pub selected : bool, 
    pub moved : bool, 
}

pub struct Bishop {
    pub board_coords_x : u8, 
    pub board_coords_y : u8, 
    pub selected : bool,     
}

pub struct Knight {
    pub board_coords_x : u8, 
    pub board_coords_y : u8, 
    pub selected : bool, 
}

pub struct Rook {
    pub board_coords_x : u8, 
    pub board_coords_y : u8, 
    pub selected : bool, 
}

pub struct King {
    pub board_coords_x : u8, 
    pub board_coords_y : u8, 
    pub selected : bool, 
}

pub struct Queen {
    pub board_coords_x : u8, 
    pub board_coords_y : u8, 
    pub selected : bool, 
}

pub enum Piece {
    Pawn,  
    Bishop, 
    Knight, 
    Rook, 
    King, 
    Queen
}

pub struct BlackPiece;
impl Component for BlackPiece {
    type Storage = DenseVecStorage<Self>; 
}
pub struct WhitePiece; 
impl Component for WhitePiece {
    type Storage = DenseVecStorage<Self>; 
}