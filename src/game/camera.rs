use macroquad::camera::Camera2D;
use macroquad::math::Vec2;

// pub fn init_camera(pos: &Vec2, w: f32, h: f32) -> Camera2D {
//     Camera2D {
//         zoom: vec2(1., 1. * (w / h)),
//         ..Default::default()
//     }
// }

// #[derive(Clone, Copy, PartialEq)]
pub struct Camera {
    pub camera: Camera2D,
    pub width: f32,
    pub height: f32,
}

impl Camera {
    pub fn update(&mut self, player_centre: Vec2) {
        let Camera2D { zoom, .. } = self.camera;
        let w = self.width / 2.;
        let h = self.height / 2.;

        self.camera.offset.x = w;
        self.camera.offset.y = h;
        self.camera.target.x = player_centre.x;
        self.camera.target.y = player_centre.y;
    }
}
