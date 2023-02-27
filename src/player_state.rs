use libm::{cosf, sinf};
use crate::raycasting;
use crate::wasm4::gamepad::Gamepad;
use crate::wasm4::gamepad::GamepadButton::{DPadDown, DPadLeft, DPadRight, DPadUp};
use crate::wasm4::geometry::Point;
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

    pub fn update_movement(&mut self, world_map: &WorldMap, gamepad: &Gamepad) {
        let previous_position = (self.x, self.y);

        if gamepad.is_pressed(DPadUp) {
            self.x += cosf(self.camera_angle) * self.step_size;
            self.y += -sinf(self.camera_angle) * self.step_size;
        }

        if gamepad.is_pressed(DPadDown) {
            self.x -= cosf(self.camera_angle) * self.step_size;
            self.y -= -sinf(self.camera_angle) * self.step_size;
        }

        if gamepad.is_pressed(DPadRight) {
            self.camera_angle -= self.step_size;
        }

        if gamepad.is_pressed(DPadLeft) {
            self.camera_angle += self.step_size;
        }

        // If moving us on this frame put us into a wall just revert it
        if world_map.is_wall(self.x, self.y) {
            (self.x, self.y) = previous_position;
        }
    }

    /// Returns 160 wall heights from the player's perspective.
    pub fn get_view(&self, world_map: &WorldMap, fov: f32, angle_ray: f32, wall_height: f32) -> [(i32, bool); 160] {
        // The player's FOV is split in half by their viewing angle.
        // In order to get the ray's first angle we must
        // add half the FOV to the player's angle to get
        // the edge of the player's FOV.
        let starting_angle = self.camera_angle + fov / 2.0;

        let mut walls = [(0, false); 160];
        let position = self.get_position();

        for (idx, wall) in walls.iter_mut().enumerate() {
            // `idx` is what number ray we are, `wall` is
            // a mutable reference to a value in `walls`.
            let angle = starting_angle - idx as f32 * angle_ray;

            // Get both the closest horizontal and vertical wall
            // intersections for this angle.
            let h_dist = raycasting::horizontal_intersection(&position, world_map, angle);
            let v_dist = raycasting::vertical_intersection(&position, world_map, angle);

            let (_, shadow) = if h_dist < v_dist {
                (h_dist, false)
            } else {
                (v_dist, true)
            };

            // Get the minimum of the two distances and
            // "convert" it into a wall height.
            *wall = (
                (wall_height / (f32::min(h_dist, v_dist) * cosf(angle - self.camera_angle))) as i32,
                shadow
            );
        }

        walls
    }

    pub fn get_position(&self) -> Point<f32> {
        Point::new(self.x, self.y)
    }
}