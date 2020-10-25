use rand::prelude::*;

pub fn create_rand_range() -> f32{
    let mut rng = rand::thread_rng();
    rng.gen()
}