
use amethyst::{
    derive::SystemDesc,
    core::transform::Transform,
    ecs::{System, SystemData, ReadStorage, WriteStorage, Join}, 
};
use crate::pieces::piece_types::*;
use crate::highlight_circle::HighlightCircle;

#[derive(SystemDesc)]
pub struct HighlightSystem;

impl<'a> System<'a> for HighlightSystem {
    type SystemData = (
        ReadStorage<'a, WhitePawn>, 
        ReadStorage<'a, BlackPawn>, 
        ReadStorage<'a, Bishop>, 
        ReadStorage<'a, Knight>, 
        ReadStorage<'a, Rook>, 
        ReadStorage<'a, King>, 
        ReadStorage<'a, Queen>, 
        WriteStorage<'a, Transform>,
        ReadStorage<'a, HighlightCircle>
    );


    fn run(&mut self, (white_pawns, black_pawns, bishops, knights, rooks, kings, queens, mut transforms, highlights) : Self::SystemData) {
        for (_highlight, transform) in (&highlights, &mut transforms).join() {
            let mut piece_selected = false;
            for pawn in white_pawns.join() {
                if pawn.selected {
                    piece_selected = true;
                    transform.set_translation_xyz(
                        pawn.board_coords_x as f32 * 100.0 + 50.0,
                        pawn.board_coords_y as f32 * 100.0 + 50.0, 
                        0.0
                    );
                }
            }
            
            for pawn in black_pawns.join() {
                if pawn.selected {
                    piece_selected = true;
                    transform.set_translation_xyz(
                        pawn.board_coords_x as f32 * 100.0 + 50.0,
                        pawn.board_coords_y as f32 * 100.0 + 50.0, 
                        0.0
                    );
                }
            }

            for bishop in bishops.join() {
                if bishop.selected {
                    piece_selected = true;
                    transform.set_translation_xyz(
                        bishop.board_coords_x as f32 * 100.0 + 50.0, 
                        bishop.board_coords_y as f32 * 100.0 + 50.0, 
                        0.0
                    );
                }
            }

            for knight in knights.join() {
                if knight.selected {
                    piece_selected = true;
                    transform.set_translation_xyz(
                        knight.board_coords_x as f32 * 100.0 + 50.0, 
                        knight.board_coords_y as f32 * 100.0 + 50.0, 
                        0.0
                    );
                }
            }

            for rook in rooks.join() {
                if rook.selected {
                    piece_selected = true;
                    transform.set_translation_xyz(
                        rook.board_coords_x as f32 * 100.0 + 50.0, 
                        rook.board_coords_y as f32 * 100.0 + 50.0, 
                        0.0
                    );
                }
            }

            for queen in queens.join() {
                if queen.selected {
                    piece_selected = true;
                    transform.set_translation_xyz(
                        queen.board_coords_x as f32 * 100.0 + 50.0,
                        queen.board_coords_y as f32 * 100.0 + 50.0, 
                        0.0
                    );
                }
            }

            for king in kings.join() {
                if king.selected {
                    piece_selected = true;
                    transform.set_translation_xyz(
                        king.board_coords_x as f32 * 100.0 + 50.0, 
                        king.board_coords_y as f32 * 100.0 + 50.0, 
                        0.0
                    );
                }
            }

            if !piece_selected{
                transform.set_translation_xyz(10.0 * 100.0 + 50.0, 10.0 * 100.0 + 50.0, 0.0);
            }
        } 
    }
}