use libm::cosf;
use crate::camera::Camera;
use crate::Color;
use crate::ray_casting::{Direction, ray_cast};
use crate::ray_casting::Direction::Horizontal;
use crate::renderable::Renderable;
use crate::wasm4::framebuffer::Framebuffer;
use crate::wasm4::geometry::{Point, Size};
use crate::world_map::WorldMap;

pub struct WorldMapRender<'a> {
    world_map: &'a WorldMap,
    camera: &'a Camera,
}

impl<'a> WorldMapRender<'a> {
    pub fn new(world_map: &'a WorldMap, camera: &'a Camera) -> Self {
        Self { world_map, camera }
    }
}

impl Renderable for WorldMapRender<'_> {
    fn render(&self, framebuffer: &Framebuffer) {
        framebuffer.set_color(Color::Ground as u16);
        framebuffer.rectangle(Point::new(0, 80), Size::new(160, 160));

        let view = self.get_view(self.camera.fov / 160.0, 100.0);
        for (x, wall) in view.iter().enumerate() {
            let (wall_height, direction) = wall;
            framebuffer.set_color(
                if *direction == Horizontal {
                    Color::WallShadowed
                } else {
                    Color::WallNormal
                } as u16
            );

            framebuffer.line_vertical(
                Point::new(x as i32, 80 - (wall_height / 2)),
                *wall_height as u32,
            );
        }
    }
}

impl WorldMapRender<'_> {
    /// Returns 160 wall heights from the player's perspective.
    fn get_view(&self, angle_ray: f32, wall_height: f32) -> [(i32, Direction); 160] {
        // The player's FOV is split in half by their viewing angle.
        // In order to get the ray's first angle we must
        // add half the FOV to the player's angle to get
        // the edge of the player's FOV.
        let starting_angle = self.camera.angle + self.camera.fov / 2.0;

        let mut walls = [(0, Direction::Horizontal); 160];

        for (idx, wall) in walls.iter_mut().enumerate() {
            // `idx` is what number ray we are, `wall` is
            // a mutable reference to a value in `walls`.
            let angle = starting_angle - idx as f32 * angle_ray;

            // Get both the closest wall intersections for this angle
            let (distance, direction) = ray_cast(&self.camera.position, self.world_map, angle);

            // Get the minimum of the two distances and
            // "convert" it into a wall height.
            *wall = (
                (wall_height / (distance * cosf(angle - self.camera.angle))) as i32,
                direction
            );
        }

        walls
    }
}