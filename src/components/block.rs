use amethyst::{
    core::Transform,
    core::ecs::{LazyUpdate, Entities, DenseVecStorage, Component},
    core::math::{Vector3, Vector2},
    shred::ReadExpect,
    prelude::*,
    renderer::SpriteRender,
};
use rand::prelude::*;

use crate::asset::sprite_sheet;
use crate::components::object::Object;
use crate::util::rand::*;

const BLOCK_HEIGHT: f32 = 60.0;
const BLOCK_WIDTH: f32 = 60.0;
const BLOCK_SCALE: f32 = 0.6;
const BLOCK_SPEED_BASE: f32 = 3.;
const BLOCK_START_X_MIN: f32 = 0.0;
const BLOCK_START_X_MAX: f32 = 960.0;
const BLOCK_START_Y: f32 = 540.0;

#[derive(Clone)]
pub struct BlockResource {
    pub block: Block,
    pub sprite: SpriteRender,
}

impl Component for BlockResource {
    type Storage = DenseVecStorage<Self>;
}

#[derive(Clone)]
pub struct Block {
    pub size: Vector2<f32>,
    pub velocity: Vector2<f32>,
    pub is_rock: bool,
}

impl Block {
    fn new(width: f32, height: f32, is_rock: bool) -> Block {
        Block {
            size: Vector2::new(width, height),
            velocity: Vector2::new(0., 0.),
            is_rock: is_rock
        }
    }
}

impl Object for Block {
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

impl Component for Block {
    type Storage = DenseVecStorage<Self>;
}

pub fn create_block(resource: &BlockResource, update: &LazyUpdate, entities: &Entities) {
    let entity = entities.create();
    
    let mut block_transform = Transform::default();
    let rnd_position = create_rand_range();
    let block_position_x: f32 = BLOCK_START_X_MIN + (BLOCK_START_X_MAX - BLOCK_START_X_MIN) * rnd_position;
    block_transform.set_translation_xyz(block_position_x, BLOCK_START_Y, 0.0);
    
    let rnd_speed = create_rand_range();
    let mut block = resource.block.clone();
    block.set_velocity(0., rnd_speed * BLOCK_SPEED_BASE);
    block_transform.set_scale(Vector3::new(BLOCK_SCALE, BLOCK_SCALE, 1.0));
    
    update.insert(entity, block);
    update.insert(entity, resource.sprite.clone());
    update.insert(entity, block_transform);
}

pub fn initialize(world: &mut World) {
    initialize_apple_resource(world);
    initialize_rock_resource(world);
}

fn initialize_apple_resource(world: &mut World) {
    let resource = BlockResource {
        block: Block::new(BLOCK_WIDTH, BLOCK_HEIGHT, false),
        sprite: create_apple_sprite(world).clone(),
    };
    world.insert(resource.clone());
}

fn initialize_rock_resource(world: &mut World) {
    let resource = BlockResource {
        block: Block::new(BLOCK_WIDTH * BLOCK_SCALE, BLOCK_HEIGHT * BLOCK_SCALE, true),
        sprite: create_rock_sprite(world).clone(),
    };
    world.insert(resource.clone());
}

fn create_apple_sprite(world: &mut World) -> SpriteRender {
    sprite_sheet::create_sprite("texture/apple_spritesheet.png", "texture/apple_spritesheet.ron", 0, world)
}

fn create_rock_sprite(world: &mut World) -> SpriteRender {
    sprite_sheet::create_sprite("texture/rock_spritesheet.png", "texture/rock_spritesheet.ron", 0, world)
}
