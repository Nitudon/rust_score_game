use amethyst::{
    core::Transform,
    core::ecs::{LazyUpdate, Entities, DenseVecStorage, Component},
    core::math::{Vector3, Vector2},
    prelude::*,
    renderer::SpriteRender,
    shred::ReadExpect
};

use crate::asset::sprite_sheet;
use crate::components::object::Object;

#[derive(Clone)]
pub struct PlayerResource {
    pub player: Player,
    pub sprite: SpriteRender,
}

#[derive(Clone)]
pub struct Player {
    pub size: Vector2<f32>,
    pub velocity: Vector2<f32>,
}

pub const PLAYER_HEIGHT: f32 = 90.0;
pub const PLAYER_WIDTH: f32 = 90.0;
pub const PLAYER_START_X: f32 = 480.0;
pub const PLAYER_START_Y: f32 = 60.0;

impl Player {
    pub fn new(width : f32, height : f32) -> Player {
        Player {
            size: Vector2::new(width, height),
            velocity: Vector2::new(0., 0.)
        }
    }
}

impl Object for Player {
    fn get_size(&self) -> &Vector2<f32> {
        &self.size
    }
    
    fn set_size(&mut self, width: f32, height: f32) {
        self.size.x = width;
        self.size.y = height;
    }

    fn get_velocity(&self) -> &Vector2<f32> {
        &self.velocity
    }

    fn set_velocity(&mut self, x: f32, y: f32) {
        self.velocity.x = x;
        self.velocity.y = y;
    }
}

impl Component for Player {
    type Storage = DenseVecStorage<Self>;
}

pub fn initialize(world: &mut World) {
    let player_sprite = create_player_sprite(world);
    let mut player_component = Player::new(PLAYER_WIDTH, PLAYER_HEIGHT);
    let mut player_transform = Transform::default();

    player_transform.set_translation_xyz(PLAYER_START_X, PLAYER_START_Y, -5.0);
    player_transform.set_scale(Vector3::new(0.5, 0.5, 1.0));

    world
        .create_entity()
        .with(player_sprite.clone())
        .with(player_component.clone())
        .with(player_transform)
        .build();
}

pub fn create_player(resource: &ReadExpect<PlayerResource>, update: &ReadExpect<LazyUpdate>, entities: &Entities) {
    let entity = entities.create();
    let mut player_transform = Transform::default();

    player_transform.set_translation_xyz(PLAYER_START_X, PLAYER_START_Y, 0.0);
    player_transform.set_scale(Vector3::new(0.5, 0.5, 1.0));

    update.insert(entity, player_transform);
    update.insert(entity, resource.player.clone());
    update.insert(entity, resource.sprite.clone());
}

fn create_player_sprite(world: &mut World) -> SpriteRender {
    sprite_sheet::create_sprite("texture/player_spritesheet.png", "texture/player_spritesheet.ron", 0, world)
}
