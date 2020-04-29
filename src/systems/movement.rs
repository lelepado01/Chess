
use amethyst::{
    input::{InputHandler, StringBindings},
    renderer::rendy::wsi::winit::MouseButton,
    ecs::{Read, ReadStorage, WriteStorage, System, SystemData, Join, Write}, 
    derive::SystemDesc, 
    core::transform::Transform,
};

use crate::pieces::piece_types::*;
use crate::states::TurnProgress;

#[derive(SystemDesc)]
pub struct WhiteMovementSystem;

#[derive(SystemDesc)]
pub struct BlackMovementSystem;

impl<'a> System<'a> for WhiteMovementSystem {
    type SystemData = (
        ReadStorage<'a, WhitePiece>, 
        WriteStorage<'a, Bishop>, 
        WriteStorage<'a, King>, 
        WriteStorage<'a, Queen>, 
        WriteStorage<'a, Knight>, 
        WriteStorage<'a, WhitePawn>, 
        WriteStorage<'a, Rook>, 
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler<StringBindings>>, 
        Write<'a, TurnProgress>,
    ); 

    fn run(&mut self, (white_pieces, mut bishops, mut kings, mut queens, mut knights, mut white_pawn, mut rooks, mut transforms, input, mut turn) : Self::SystemData) {
        if input.mouse_button_is_down(MouseButton::Left){
            if let Some((x, y)) = input.mouse_position() {
                let board_x = (x / 200.0) as u8;
                let board_y = 7 - (y / 200.0) as u8;
        
                for (pawn, transform) in (&mut white_pawn, &mut transforms).join() {
                    if pawn.selected && pawn.move_is_available(board_x, board_y){
                        pawn.board_coords_x = board_x; 
                        pawn.board_coords_y = board_y;
                        pawn.moved = true; 
                        pawn.selected = false;
                        transform.set_translation_xyz(
                            100.0 * (pawn.board_coords_x as f32) + 45.0, 
                            100.0 * (pawn.board_coords_y as f32) + 50.0, 
                            0.1
                        );

                        turn.moved = true; 
                    }
                }

                for (bishop, transform, _white_piece) in (&mut bishops, &mut transforms, &white_pieces).join() {
                    if bishop.selected && bishop.move_is_available(board_x, board_y){
                        bishop.board_coords_x = board_x; 
                        bishop.board_coords_y = board_y;
                        bishop.selected = false;
                        transform.set_translation_xyz(
                            100.0 * (bishop.board_coords_x as f32) + 45.0, 
                            100.0 * (bishop.board_coords_y as f32) + 50.0, 
                            0.1
                        );

                        turn.moved = true;
                    }
                }

                for (king, transform, _white_piece) in (&mut kings, &mut transforms, &white_pieces).join() {
                    if king.selected && king.move_is_available(board_x, board_y){
                        king.board_coords_x = board_x; 
                        king.board_coords_y = board_y;
                        king.selected = false;
                        transform.set_translation_xyz(
                            100.0 * (king.board_coords_x as f32) + 45.0, 
                            100.0 * (king.board_coords_y as f32) + 50.0, 
                            0.1
                        );

                        turn.moved = true;
                    }
                }

                for (rook, transform, _white_piece) in (&mut rooks, &mut transforms, &white_pieces).join() {
                    if rook.selected && rook.move_is_available(board_x, board_y){
                        rook.board_coords_x = board_x; 
                        rook.board_coords_y = board_y;
                        rook.selected = false;
                        transform.set_translation_xyz(
                            100.0 * (rook.board_coords_x as f32) + 45.0, 
                            100.0 * (rook.board_coords_y as f32) + 50.0, 
                            0.1
                        );

                        turn.moved = true;
                    }
                }

                for (queen, transform, _white_piece) in (&mut queens, &mut transforms, &white_pieces).join() {
                    if queen.selected && queen.move_is_available(board_x, board_y){
                        queen.board_coords_x = board_x; 
                        queen.board_coords_y = board_y;
                        queen.selected = false;
                        transform.set_translation_xyz(
                            100.0 * (queen.board_coords_x as f32) + 45.0, 
                            100.0 * (queen.board_coords_y as f32) + 50.0, 
                            0.1
                        );

                        turn.moved = true;
                    }
                }

                for (knight, transform, _white_piece) in (&mut knights, &mut transforms, &white_pieces).join() {
                    if knight.selected && knight.move_is_available(board_x, board_y){
                        knight.board_coords_x = board_x; 
                        knight.board_coords_y = board_y;
                        knight.selected = false;
                        transform.set_translation_xyz(
                            100.0 * (knight.board_coords_x as f32) + 45.0, 
                            100.0 * (knight.board_coords_y as f32) + 50.0, 
                            0.1
                        );

                        turn.moved = true;
                    }
                }
            }
        }
    }
}

impl<'a> System<'a> for BlackMovementSystem {
    type SystemData = (
        ReadStorage<'a, BlackPiece>, 
        WriteStorage<'a, Bishop>, 
        WriteStorage<'a, King>, 
        WriteStorage<'a, Queen>, 
        WriteStorage<'a, Knight>, 
        WriteStorage<'a, BlackPawn>, 
        WriteStorage<'a, Rook>, 
        WriteStorage<'a, Transform>,
        Read<'a, InputHandler<StringBindings>>, 
        Write<'a, TurnProgress>, 
    ); 

    fn run(&mut self, (black_pieces, mut bishops, mut kings, mut queens, mut knights, mut white_pawn, mut rooks, mut transforms, input, mut turn) : Self::SystemData) {
        if input.mouse_button_is_down(MouseButton::Left){
            if let Some((x, y)) = input.mouse_position() {
                let board_x = (x / 200.0) as u8;
                let board_y = 7 - (y / 200.0) as u8;
        
                for (pawn, transform) in (&mut white_pawn, &mut transforms).join() {
                    if pawn.selected && pawn.move_is_available(board_x, board_y){
                        pawn.board_coords_x = board_x; 
                        pawn.board_coords_y = board_y;
                        pawn.moved = true; 
                        pawn.selected = false;
                        transform.set_translation_xyz(
                            100.0 * (pawn.board_coords_x as f32) + 45.0, 
                            100.0 * (pawn.board_coords_y as f32) + 50.0, 
                            0.1
                        );

                        turn.moved = true;
                    }
                }

                for (bishop, transform, _white_piece) in (&mut bishops, &mut transforms, &black_pieces).join() {
                    if bishop.selected && bishop.move_is_available(board_x, board_y){
                        bishop.board_coords_x = board_x; 
                        bishop.board_coords_y = board_y;
                        bishop.selected = false;
                        transform.set_translation_xyz(
                            100.0 * (bishop.board_coords_x as f32) + 45.0, 
                            100.0 * (bishop.board_coords_y as f32) + 50.0, 
                            0.1
                        );

                        turn.moved = true;
                    }
                }

                for (king, transform, _white_piece) in (&mut kings, &mut transforms, &black_pieces).join() {
                    if king.selected && king.move_is_available(board_x, board_y){
                        king.board_coords_x = board_x; 
                        king.board_coords_y = board_y;
                        king.selected = false;
                        transform.set_translation_xyz(
                            100.0 * (king.board_coords_x as f32) + 45.0, 
                            100.0 * (king.board_coords_y as f32) + 50.0, 
                            0.1
                        );

                        turn.moved = true;
                    }
                }

                for (rook, transform, _black_piece) in (&mut rooks, &mut transforms, &black_pieces).join() {
                    if rook.selected && rook.move_is_available(board_x, board_y){
                        rook.board_coords_x = board_x; 
                        rook.board_coords_y = board_y;
                        rook.selected = false;
                        transform.set_translation_xyz(
                            100.0 * (rook.board_coords_x as f32) + 45.0, 
                            100.0 * (rook.board_coords_y as f32) + 50.0, 
                            0.1
                        );

                        turn.moved = true;
                    }
                }

                for (queen, transform, _black_piece) in (&mut queens, &mut transforms, &black_pieces).join() {
                    if queen.selected && queen.move_is_available(board_x, board_y){
                        queen.board_coords_x = board_x; 
                        queen.board_coords_y = board_y;
                        queen.selected = false;
                        transform.set_translation_xyz(
                            100.0 * (queen.board_coords_x as f32) + 45.0, 
                            100.0 * (queen.board_coords_y as f32) + 50.0, 
                            0.1
                        );

                        turn.moved = true;
                    }
                }

                for (knight, transform, _black_piece) in (&mut knights, &mut transforms, &black_pieces).join() {
                    if knight.selected && knight.move_is_available(board_x, board_y){
                        knight.board_coords_x = board_x; 
                        knight.board_coords_y = board_y;
                        knight.selected = false;
                        transform.set_translation_xyz(
                            100.0 * (knight.board_coords_x as f32) + 45.0, 
                            100.0 * (knight.board_coords_y as f32) + 50.0, 
                            0.1
                        );

                        turn.moved = true;
                    }
                }
            }
        }
    }
}