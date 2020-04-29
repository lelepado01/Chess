
use amethyst::{
    prelude::*, 
    assets::{Loader, AssetStorage},
    renderer::{
        ImageFormat, Texture, SpriteRender, SpriteSheet, SpriteSheetFormat
    }, 
    core::transform::Transform, 
    ecs::{Component, DenseVecStorage}
};

pub struct Board {
    pub board_width : f32, 
    pub board_height : f32, 
}

pub const BOARD : u8 = 8;  

impl Component for Board {
    type Storage = DenseVecStorage<Self>; 
}

pub fn initialize_board(world : &mut World) {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let board_storage = world.read_resource::<AssetStorage<Texture>>();

        loader.load(
            "textures/board.png", 
            ImageFormat::default(), 
            (), 
            &board_storage
        )
    };

    let sprite_handle = {
        let loader = world.read_resource::<Loader>();
        let storage = world.read_resource::<AssetStorage<SpriteSheet>>();

        loader.load(
            "board.ron", 
            SpriteSheetFormat(texture_handle), 
            (), 
            &storage
        )
    };

    let mut transform = Transform::default();
    transform.set_translation_xyz(400.0, 400.0, 0.0);

    let board = Board{
        board_width : BOARD as f32 * 100.0,
        board_height : BOARD as f32 * 100.0
    };

    let sprite = SpriteRender {
        sprite_sheet : sprite_handle, 
        sprite_number : 0, 
    };

    world
        .create_entity()
        .with(transform)
        .with(board)
        .with(sprite)
        .build();
}