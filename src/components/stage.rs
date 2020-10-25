use amethyst::{
    core::transform::Transform,
    prelude::*,
    renderer::{Camera, SpriteRender},
    core::math::Vector3,
};
use crate::asset::sprite_sheet;

pub const SCREEN_HEIGHT: f32 = 540.0;
pub const SCREEN_WIDTH: f32 = 960.0;

pub fn initialize(world: &mut World) {
    let mut camera_transform = Transform::default();
    let camera_component = Camera::standard_2d(SCREEN_WIDTH, SCREEN_HEIGHT);
    camera_transform.set_translation_xyz(SCREEN_WIDTH * 0.5, SCREEN_HEIGHT * 0.5, 1.0);

    world
        .create_entity()
        .with(camera_component)
        .with(camera_transform)
        .build();

    let stage_sprite = create_stage_sprite(world);
    let mut stage_transform = Transform::default();
    stage_transform.set_translation_xyz(SCREEN_WIDTH * 0.5, SCREEN_HEIGHT * 0.5, -10.0);
    stage_transform.set_scale(Vector3::new(0.6, 0.6, 1.0));

    world
        .create_entity()
        .with(stage_sprite)
        .with(stage_transform)
        .build();
}

fn create_stage_sprite(world: &mut World) -> SpriteRender {
    sprite_sheet::create_sprite("texture/stage_spritesheet.png", "texture/stage_spritesheet.ron", 0, world)
}