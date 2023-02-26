use libm::{cosf, sinf};
use crate::world_map::WorldMap;

pub struct PlayerState {
    x: f32,
    y: f32,
    camera_angle: f32,
    pub step_size: f32,
}

impl PlayerState {
    pub const fn new(x: f32, y: f32, camera_angle: f32) -> Self {
        Self { x, y, camera_angle, step_size: 0.045 }
    }

    pub fn update(&mut self, world_map: &WorldMap, up: bool, down: bool, left: bool, right: bool) {
        let previous_position = (self.x, self.y);

        if up {
            self.x += cosf(self.camera_angle) * self.step_size;
            self.y += -sinf(self.camera_angle) * self.step_size;
        }

        if down {
            self.x -= cosf(self.camera_angle) * self.step_size;
            self.y -= -sinf(self.camera_angle) * self.step_size;
        }

        if right {
            self.camera_angle -= self.step_size;
        }

        if left {
            self.camera_angle += self.step_size;
        }

        // If moving us on this frame put us into a wall just revert it
        if world_map.is_wall(self.x, self.y) {
            (self.x, self.y) = previous_position;
        }
    }
}