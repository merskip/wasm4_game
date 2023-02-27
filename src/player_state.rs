use libm::{cosf, sinf};
use crate::camera::Camera;
use crate::wasm4::gamepad::Gamepad;
use crate::wasm4::gamepad::GamepadButton::{DPadDown, DPadLeft, DPadRight, DPadUp};
use crate::world_map::WorldMap;

pub struct PlayerState {
    pub camera: Camera,
    pub step_size: f32,
}

impl PlayerState {
    pub const fn new(camera: Camera, step_size: f32) -> Self {
        Self { camera, step_size }
    }

    pub fn update_movement(&mut self, world_map: &WorldMap, gamepad: &Gamepad) {
        let previous_position = (self.camera.position.x, self.camera.position.y);

        if gamepad.is_pressed(DPadUp) {
            self.camera.position.x += cosf(self.camera.angle) * self.step_size;
            self.camera.position.y += -sinf(self.camera.angle) * self.step_size;
        }

        if gamepad.is_pressed(DPadDown) {
            self.camera.position.x -= cosf(self.camera.angle) * self.step_size;
            self.camera.position.y -= -sinf(self.camera.angle) * self.step_size;
        }

        if gamepad.is_pressed(DPadRight) {
            self.camera.angle -= self.step_size;
        }

        if gamepad.is_pressed(DPadLeft) {
            self.camera.angle += self.step_size;
        }

        // If moving us on this frame put us into a wall just revert it
        if world_map.is_wall(self.camera.position.x, self.camera.position.y) {
            (self.camera.position.x, self.camera.position.y) = previous_position;
        }
    }
}