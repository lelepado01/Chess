
use amethyst::{
    prelude::*, 
    core::transform::Transform,
    core::math::Vector3,
    ecs::{DenseVecStorage, Component}, 
    renderer::{SpriteRender, SpriteSheet, ImageFormat, Texture, SpriteSheetFormat}, 
    assets::{Loader, AssetStorage}
};

pub struct HighlightCircle;

impl Component for HighlightCircle {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize_highlight(world : &mut World) {
    let texture_handle = {
        let loader = world.read_resource::<Loader>();
        let storage = world.read_resource::<AssetStorage<Texture>>();
        
        loader.load(
            "textures/highlight_rect.png", 
            ImageFormat::default(), 
            (), 
            &storage
        )
    };

    let sprite_handle = {
        let loader = world.read_resource::<Loader>();
        let highlight_storage = world.read_resource::<AssetStorage<SpriteSheet>>();

        loader.load(
            "highlight_rect.ron",
            SpriteSheetFormat(texture_handle),
            (), 
            &highlight_storage
        )
    }; 

    let mut transform = Transform::default();
    transform.set_translation_xyz(0.0, 0.0, 0.1);
    transform.set_scale(Vector3::new(0.23, 0.38, 0.0));

    let sprite = SpriteRender {
        sprite_sheet : sprite_handle, 
        sprite_number : 0,
    };

    world 
        .create_entity()
        .with(transform)
        .with(HighlightCircle{})
        .with(sprite)
        .build();
}