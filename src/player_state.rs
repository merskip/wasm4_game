pub struct PlayerState {
    x: f32,
    y: f32,
    camera_angle: f32,
}

impl PlayerState {
    pub const fn new(x: f32, y: f32, camera_angle: f32) -> Self {
        Self { x, y, camera_angle }
    }
}