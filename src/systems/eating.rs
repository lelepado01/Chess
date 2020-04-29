
use amethyst::{
    ecs::{Entities, ReadStorage, System, SystemData, Join}, 
    derive::SystemDesc, 
    core::transform::Transform,
};

use crate::pieces::piece_types::*;

#[derive(SystemDesc)]
pub struct WhiteEatingSystem;
#[derive(SystemDesc)]
pub struct BlackEatingSystem;

impl<'a> System<'a> for WhiteEatingSystem {
    type SystemData = (
        ReadStorage<'a, WhitePiece>, 
        ReadStorage<'a, BlackPiece>, 
        ReadStorage<'a, Transform>,
        Entities<'a>,
    );

    fn run(&mut self, (white_pieces, black_pieces, transforms, entities) : Self::SystemData) {
        for (_, white_transform, _) in (&white_pieces, &transforms, &entities).join() {
            for (_, black_transform, entity) in (&black_pieces, &transforms, &entities).join() {
                if white_transform.translation() == black_transform.translation() {
                    let _ = entities.delete(entity);
                }
            }
        }
    }
}

impl<'a> System<'a> for BlackEatingSystem {
    type SystemData = (
        ReadStorage<'a, WhitePiece>, 
        ReadStorage<'a, BlackPiece>, 
        ReadStorage<'a, Transform>,
        Entities<'a>,
    );

    fn run(&mut self, (white_pieces, black_pieces, transforms, entities) : Self::SystemData) {
        for (_, white_transform, _) in (&black_pieces, &transforms, &entities).join() {
            for (_, black_transform, entity) in (&white_pieces, &transforms, &entities).join() {
                if white_transform.translation() == black_transform.translation() {
                    let _ = entities.delete(entity);
                }
            }
        }
    }
}