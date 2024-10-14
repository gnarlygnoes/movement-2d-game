use macroquad::{
    input::{is_key_down, KeyCode},
    math::Vec2,
};
use macroquad_platformer::Actor;

pub struct Mover {
    pub collider: Actor,
    pub speed: Vec2,
    pub size: f32,
}

impl Mover {
    fn move_mover(self) -> Vec2 {
        if is_key_down(KeyCode::Space) {
            return Vec2 { x: 0., y: -1. };
        }
        return Vec2 { x: 0., y: 0. };
    }
}
