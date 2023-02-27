use crate::wasm4::geometry::Point;

pub struct Camera {
    pub position: Point<f32>,
    pub angle: f32,
    pub fov: f32
}

impl Camera {
    pub fn new(position: Point<f32>, angle: f32, fov: f32) -> Self {
        Self {
            position,
            angle,
            fov
        }
    }
}
