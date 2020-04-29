
use amethyst::{
    core::transform::Transform, 
    core::math::Vector3,
    prelude::*, 
    renderer::{
        SpriteSheet, SpriteSheetFormat, ImageFormat, SpriteRender, Texture
    }, 
    assets::{Handle, AssetStorage, Loader}, 
};

use super::piece_types::*;

pub fn initialize_pieces(world : &mut World) {
    let sprite_handle = load_textures(world); 
    
    initialize_pawns(world, sprite_handle.clone());
    initialize_knights(world, sprite_handle.clone());
    initialize_bishops(world, sprite_handle.clone());
    initialize_rooks(world, sprite_handle.clone()); 
    initialize_king(world, sprite_handle.clone());
    initialize_queen(world, sprite_handle.clone());
}

fn load_textures(world : &mut World) -> Handle<SpriteSheet> {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let storage = world.read_resource::<AssetStorage<Texture>>(); 

        loader.load(
            "textures/pieces.png", 
            ImageFormat::default(), 
            (), 
            &storage
        )
    }; 

    let sprite_handle = {
        let loader = world.read_resource::<Loader>(); 
        let storage = world.read_resource::<AssetStorage<SpriteSheet>>(); 

        loader.load(
            "pieces.ron", 
            SpriteSheetFormat(texture_handle), 
            (), 
            &storage
        )
    };

    sprite_handle
}

fn initialize_white_piece(world : &mut World, sprite : SpriteRender, piece_type : Piece, x : u8, y : u8){
    let mut transform = Transform::default(); 
    transform.set_translation_xyz(100.0 * (x as f32) + 45.0, 100.0 * (y as f32) + 50.0, 0.1); 
    transform.set_scale(Vector3::new(0.25, 0.25, 0.0));

    match piece_type {
        Piece::Bishop =>
            world
                .create_entity()
                .with(transform)
                .with(sprite)
                .with(WhitePiece{})
                .with(Bishop {board_coords_x : x, board_coords_y : y, selected : false})
                .build(), 
        Piece::King => { 
            world
                .create_entity()
                .with(transform)
                .with(sprite)
                .with(WhitePiece{})
                .with(King {board_coords_x : x, board_coords_y : y, selected : false})
                .build()}, 
        Piece::Knight => 
            world
                .create_entity()
                .with(transform)
                .with(sprite)
                .with(WhitePiece{})
                .with(Knight {board_coords_x : x, board_coords_y : y, selected : false})
                .build(), 
        Piece::Pawn => 
            world
                .create_entity()
                .with(transform)
                .with(sprite)
                .with(WhitePiece{})
                .with(WhitePawn {board_coords_x : x, board_coords_y : y, selected : false, moved : false})
                .build(), 
        Piece::Queen => 
            world
                .create_entity()
                .with(transform)
                .with(sprite)
                .with(WhitePiece{})
                .with(Queen {board_coords_x : x, board_coords_y : y, selected : false})
                .build(), 
        _ => 
            world
                .create_entity()
                .with(transform)
                .with(sprite)
                .with(WhitePiece{})
                .with(Rook {board_coords_x : x, board_coords_y : y, selected : false})
                .build()
    };   
}

fn initialize_black_piece(world : &mut World, sprite : SpriteRender, piece_type : Piece, x : u8, y : u8){
    let mut transform = Transform::default(); 
    transform.set_translation_xyz(100.0 * (x as f32) + 45.0, 100.0 * (y as f32) + 50.0, 0.1); 
    transform.set_scale(Vector3::new(0.25, 0.25, 0.0));

    match piece_type {
        Piece::Bishop =>
            world
                .create_entity()
                .with(transform)
                .with(sprite)
                .with(BlackPiece{})
                .with(Bishop {board_coords_x : x, board_coords_y : y, selected : false})
                .build(), 
        Piece::King => 
            world
                .create_entity()
                .with(transform)
                .with(sprite)
                .with(BlackPiece{})
                .with(King {board_coords_x : x, board_coords_y : y, selected : false})
                .build(), 
        Piece::Knight => 
            world
                .create_entity()
                .with(transform)
                .with(sprite)
                .with(BlackPiece{})
                .with(Knight {board_coords_x : x, board_coords_y : y, selected : false})
                .build(), 
        Piece::Pawn => 
            world
                .create_entity()
                .with(transform)
                .with(sprite)
                .with(BlackPiece{})
                .with(BlackPawn {board_coords_x : x, board_coords_y : y, selected : false, moved : false})
                .build(),
        Piece::Queen => 
            world
                .create_entity()
                .with(transform)
                .with(sprite)
                .with(BlackPiece{})
                .with(Queen {board_coords_x : x, board_coords_y : y, selected : false})
                .build(), 
        _ => 
            world
                .create_entity()
                .with(transform)
                .with(sprite)
                .with(BlackPiece{})
                .with(Rook {board_coords_x : x, board_coords_y : y, selected : false})
                .build()
    };   
}

fn initialize_queen(world : &mut World, sprite_handle : Handle<SpriteSheet>) {
    let white_queen_sprite = SpriteRender {
        sprite_sheet : sprite_handle.clone(), 
        sprite_number : 8
    };

    let black_queen_sprite = SpriteRender {
        sprite_sheet : sprite_handle.clone(), 
        sprite_number : 9
    };

    initialize_white_piece(world, white_queen_sprite, Piece::Queen, 3, 0);
    initialize_black_piece(world, black_queen_sprite, Piece::Queen, 3, 7);
}

fn initialize_king(world : &mut World, sprite_handle : Handle<SpriteSheet>){
    let white_king_sprite = SpriteRender {
        sprite_sheet : sprite_handle.clone(), 
        sprite_number : 10
    };

    let black_king_sprite = SpriteRender {
        sprite_sheet : sprite_handle.clone(), 
        sprite_number : 11
    };
    
    initialize_white_piece(world, white_king_sprite, Piece::King, 4, 0);
    initialize_black_piece(world, black_king_sprite, Piece::King, 4, 7);
}

fn initialize_rooks(world : &mut World, sprite_handle : Handle<SpriteSheet>){
    let white_sprite = SpriteRender {
        sprite_sheet : sprite_handle.clone(), 
        sprite_number : 4
    };

    let black_sprite = SpriteRender {
        sprite_sheet : sprite_handle.clone(), 
        sprite_number : 5
    };

    initialize_white_piece(world, white_sprite.clone(), Piece::Rook, 0, 0);
    initialize_white_piece(world, white_sprite, Piece::Rook, 7, 0);
    initialize_black_piece(world, black_sprite.clone(), Piece::Rook, 0, 7);
    initialize_black_piece(world, black_sprite, Piece::Rook, 7, 7);
}

fn initialize_knights(world : &mut World, sprite_handle : Handle<SpriteSheet>){
    let white_sprite = SpriteRender {
        sprite_sheet : sprite_handle.clone(), 
        sprite_number : 2
    };

    let black_sprite = SpriteRender {
        sprite_sheet : sprite_handle.clone(), 
        sprite_number : 3
    };

    initialize_white_piece(world, white_sprite.clone(), Piece::Knight, 1, 0);
    initialize_white_piece(world, white_sprite, Piece::Knight, 6, 0);
    initialize_black_piece(world, black_sprite.clone(), Piece::Knight, 1, 7);
    initialize_black_piece(world, black_sprite, Piece::Knight, 6, 7);
}

fn initialize_bishops(world : &mut World, sprite_handle : Handle<SpriteSheet>){
    let white_sprite = SpriteRender {
        sprite_sheet : sprite_handle.clone(), 
        sprite_number : 6
    };

    let black_sprite = SpriteRender {
        sprite_sheet : sprite_handle.clone(), 
        sprite_number : 7
    };

    initialize_white_piece(world, white_sprite.clone(), Piece::Bishop, 2, 0);
    initialize_white_piece(world, white_sprite, Piece::Bishop, 5, 0);
    initialize_black_piece(world, black_sprite.clone(), Piece::Bishop, 2, 7);
    initialize_black_piece(world, black_sprite, Piece::Bishop, 5, 7);
}

fn initialize_pawns(world : &mut World, sprite_handle : Handle<SpriteSheet>) {
    let white_sprite = SpriteRender {
        sprite_sheet : sprite_handle.clone(), 
        sprite_number : 0
    };

    let black_sprite = SpriteRender {
        sprite_sheet : sprite_handle.clone(), 
        sprite_number : 1
    };

    for i in 0..8 {
        initialize_white_piece(world, white_sprite.clone(), Piece::Pawn, i, 1);
        initialize_black_piece(world, black_sprite.clone(), Piece::Pawn, i, 6);
    }
}