
use amethyst::{
    prelude::*, 
    renderer::Camera, 
    core::transform::Transform
};


pub fn initialize_camera(world : &mut World) {
    let mut transform = Transform::default(); 
    transform.set_translation_xyz(400.0, 400.0, 1.0);

    world
        .create_entity()
        .with(transform)
        .with(Camera::standard_2d(800.0, 800.0))
        .build();
}