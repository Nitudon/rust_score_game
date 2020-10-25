use amethyst::{
    core::num::abs,
    core::math::Vector2,
    core::Transform 
};

pub trait Object {
    fn get_size(&self) -> &Vector2<f32>;
    fn set_size(&mut self, width: f32, height: f32);
    fn get_velocity(&self) -> &Vector2<f32>;
    fn set_velocity(&mut self, x: f32, y: f32);
}

pub fn is_hit(obj1 : &Transform, size1 : &Vector2<f32>, obj2: &Transform, size2 : &Vector2<f32>) -> bool {
    let distance_x = abs(obj1.translation().x - obj2.translation().x);
    let distance_y = abs(obj1.translation().y - obj2.translation().y);

    distance_x <= size1.x / 2. + size2.x / 2. && distance_y <= size1.y / 2. + size2.y / 2.
}