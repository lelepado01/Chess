use amethyst::{
    input::{InputHandler, StringBindings},
    renderer::rendy::wsi::winit::MouseButton,
    derive::SystemDesc,
    ecs::{Read, System, SystemData, ReadStorage, WriteStorage, Join}, 
};
use crate::pieces::piece_types::*;

#[derive(SystemDesc)]
pub struct WhiteSelectionSystem;

#[derive(SystemDesc)]
pub struct BlackSelectionSystem;

impl<'a> System<'a> for WhiteSelectionSystem {
    type SystemData = (
        ReadStorage<'a, WhitePiece>, 
        WriteStorage<'a, Bishop>, 
        WriteStorage<'a, King>, 
        WriteStorage<'a, Queen>, 
        WriteStorage<'a, Knight>, 
        WriteStorage<'a, WhitePawn>, 
        WriteStorage<'a, Rook>, 
        Read<'a, InputHandler<StringBindings>>, 
    ); 

    fn run(&mut self, (white_pieces, mut bishops, mut kings, mut queens, mut knights, mut pawns, mut rooks, input) : Self::SystemData) {
        if input.mouse_button_is_down(MouseButton::Left){
            if let Some((x, y)) = input.mouse_position() {
                let board_x = (x / 200.0) as u8;
                let board_y = 7 - (y / 200.0) as u8;

                for (pawn, _black_piece) in (&mut pawns, &white_pieces).join() {
                    if board_x == pawn.board_coords_x && board_y == pawn.board_coords_y {
                        pawn.selected = true;
                    } else {
                        if pawn.selected && !pawn.move_is_available(board_x, board_y){
                            pawn.selected = false;
                        }
                    }
                }

                for (knight, _black_piece) in (&mut knights, &white_pieces).join() {
                    if knight.board_coords_x == board_x && board_y == knight.board_coords_y {
                        knight.selected = true;
                    } else {
                        if knight.selected && !knight.move_is_available(board_x, board_y) {
                            knight.selected = false;
                        }
                    }
                }

                for (bishop, _black_piece) in (&mut bishops, &white_pieces).join() {
                    if (board_x, board_y) == (bishop.board_coords_x, bishop.board_coords_y) {
                        bishop.selected = true;
                    } else {
                        if bishop.selected && !bishop.move_is_available(board_x, board_y){
                            bishop.selected = false;
                        }
                    }
                }

                for (rook, _black_piece) in (&mut rooks, &white_pieces).join() {
                    if (board_x, board_y) == (rook.board_coords_x, rook.board_coords_y) {
                        rook.selected = true;
                    } else {
                        if rook.selected && !rook.move_is_available(board_x, board_y){
                            rook.selected = false;
                        }
                    }
                }

                for (king, _black_piece) in (&mut kings, &white_pieces).join() {
                    if (board_x, board_y) == (king.board_coords_x, king.board_coords_y) {
                        king.selected = true;
                    } else {
                        if king.selected && !king.move_is_available(board_x, board_y){
                            king.selected = false;
                        }
                    }
                }

                for (queen, _black_piece) in (&mut queens, &white_pieces).join() {
                    if (board_x, board_y) == (queen.board_coords_x, queen.board_coords_y) {
                        queen.selected = true;
                    } else {
                        if queen.selected && !queen.move_is_available(board_x, board_y){
                            queen.selected = false;
                        }
                    }
                }

            }
        }
    }
}

impl<'a> System<'a> for BlackSelectionSystem {
    type SystemData = (
        ReadStorage<'a, BlackPiece>, 
        WriteStorage<'a, Bishop>, 
        WriteStorage<'a, King>, 
        WriteStorage<'a, Queen>, 
        WriteStorage<'a, Knight>, 
        WriteStorage<'a, BlackPawn>, 
        WriteStorage<'a, Rook>, 
        Read<'a, InputHandler<StringBindings>>, 
    ); 

    fn run(&mut self, (black_pieces, mut bishops, mut kings, mut queens, mut knights, mut pawns, mut rooks, input) : Self::SystemData) {
        if input.mouse_button_is_down(MouseButton::Left){
            if let Some((x, y)) = input.mouse_position() {
                let board_x = (x / 200.0) as u8;
                let board_y = 7 - (y / 200.0) as u8;

                for (pawn, _black_piece) in (&mut pawns, &black_pieces).join() {
                    if board_x == pawn.board_coords_x && board_y == pawn.board_coords_y {
                        pawn.selected = true;
                    } else {
                        if pawn.selected && !pawn.move_is_available(board_x, board_y){
                            pawn.selected = false;
                        }
                    }
                }

                for (knight, _black_piece) in (&mut knights, &black_pieces).join() {
                    if knight.board_coords_x == board_x && board_y == knight.board_coords_y {
                        knight.selected = true;
                    } else {
                        if knight.selected && !knight.move_is_available(board_x, board_y){
                            knight.selected = false;
                        }
                    }
                }

                for (bishop, _black_piece) in (&mut bishops, &black_pieces).join() {
                    if (board_x, board_y) == (bishop.board_coords_x, bishop.board_coords_y) {
                        bishop.selected = true;
                    } else {
                        if bishop.selected && !bishop.move_is_available(board_x, board_y){
                            bishop.selected = false;
                        }
                    }
                }

                for (rook, _black_piece) in (&mut rooks, &black_pieces).join() {
                    if (board_x, board_y) == (rook.board_coords_x, rook.board_coords_y) {
                        rook.selected = true;
                    } else {
                        if rook.selected && !rook.move_is_available(board_x, board_y){
                            rook.selected = false;
                        }
                    }
                }

                for (king, _black_piece) in (&mut kings, &black_pieces).join() {
                    if (board_x, board_y) == (king.board_coords_x, king.board_coords_y) {
                        king.selected = true;
                    } else {
                        if king.selected && !king.move_is_available(board_x, board_y) {
                            king.selected = false;
                        }
                    }
                }

                for (queen, _black_piece) in (&mut queens, &black_pieces).join() {
                    if (board_x, board_y) == (queen.board_coords_x, queen.board_coords_y) {
                        queen.selected = true;
                    } else {
                        if queen.selected && !queen.move_is_available(board_x, board_y){
                            queen.selected = false;
                        }
                    }
                }

            }
        }
    }
}