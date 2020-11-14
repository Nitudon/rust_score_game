use amethyst::{
    core::{num, Transform},
    core::ecs::{System, WriteStorage, Read, Join},
    input::{InputHandler, StringBindings, VirtualKeyCode},
    shred::ReadExpect
};
use crate::components::player;
use crate::components::object::Object;
use crate::components::score::Score;

pub struct PlayerSystem;

pub const PLAYER_POSITION_X_MIN: f32 = 40.0;
pub const PLAYER_POSITION_X_MAX: f32 = 920.0;

impl<'a> System<'a> for PlayerSystem {
    type SystemData = (
        WriteStorage<'a, Transform>, 
        WriteStorage<'a, player::Player>,
        ReadExpect<'a, Score>,
        ReadExpect<'a, InputHandler<StringBindings>>, 
    );

    fn run(&mut self, (mut transforms, mut players, score, input): Self::SystemData) {
        if !score.is_start {
            return;
        }
        
        for (transform, player) in (&mut transforms, &mut players).join() {
            if input.key_is_down(VirtualKeyCode::Right) {
                player.set_velocity(6., 0.);
            }
            else if input.key_is_down(VirtualKeyCode::Left) {
                player.set_velocity(-6., 0.);
            }
            else { 
                player.set_velocity(0., 0.);
            }

            let player_position = num::clamp(transform.translation().x + player.get_velocity().x, PLAYER_POSITION_X_MIN, PLAYER_POSITION_X_MAX);
            transform.set_translation_x(player_position);
        }
    }
}
