#![no_main]
#![no_std]

mod world_map;
mod player_state;
mod wasm4;
mod raycasting;

use core::f32::consts::PI;
use crate::player_state::PlayerState;
use crate::wasm4::application::*;
use crate::wasm4::framebuffer::Framebuffer;
use crate::wasm4::gamepad::Gamepad;
use crate::wasm4::geometry::Point;
use crate::world_map::WorldMap;

struct MainApplication {
    world_map: WorldMap,
    player_state: PlayerState,
    gamepad: Gamepad,
    framebuffer: Framebuffer,
}

impl Application for MainApplication {
    fn start(framebuffer: Framebuffer) -> Self {
        Self {
            world_map: WorldMap::new([
                0b1111111111111111,
                0b1000001010000101,
                0b1011100000110101,
                0b1000111010010001,
                0b1010001011110111,
                0b1011101001100001,
                0b1000100000001101,
                0b1111111111111111,
            ]),
            player_state: PlayerState::new(1.5, 1.5, 0.0),
            gamepad: Gamepad::gamepad1(),
            framebuffer,
        }
    }

    fn update(&mut self) {
        self.player_state.update_movement(&self.world_map, &self.gamepad);

        let fov = PI / 2.7;
        let view = self.player_state.get_view(&self.world_map, fov, fov / 160.0, 100.0);
        for (x, wall) in view.iter().enumerate() {
            let (wall_height, shadow) = wall;
            if *shadow {
                // draw with color 2 for walls with "shadow"
                self.framebuffer.set_color(2);
            } else {
                // draw with color 3 for walls without "shadow"
                self.framebuffer.set_color(3);
            }

            self.framebuffer.line_vertical(
                Point::new(x as i32, 80 - (wall_height / 2)),
                *wall_height as u32,
            );
        }
    }
}

main_application! { MainApplication }

