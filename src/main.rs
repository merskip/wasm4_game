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
use crate::wasm4::geometry::{Point, Size};
use crate::world_map::WorldMap;

struct MainApplication {
    world_map: WorldMap,
    player_state: PlayerState,
    gamepad: Gamepad,
    framebuffer: Framebuffer,
}

#[repr(u16)]
enum Color {
    _Background = 1,
    WallNormal = 2,
    WallShadowed = 3,
    Ground = 4,
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
            player_state: PlayerState::new(1.5, 1.5, 30.0),
            gamepad: Gamepad::gamepad1(),
            framebuffer,
        }
    }

    fn update(&mut self) {
        self.player_state.update_movement(&self.world_map, &self.gamepad);

        self.framebuffer.set_color(Color::Ground as u16);
        self.framebuffer.rectangle(Point::new(0, 80), Size::new(160, 160));

        let fov = PI / 2.7;
        let view = self.player_state.get_view(&self.world_map, fov, fov / 160.0, 100.0);
        for (x, wall) in view.iter().enumerate() {
            let (wall_height, shadow) = wall;
            self.framebuffer.set_color(
                if *shadow { Color::WallShadowed } else { Color::WallNormal } as u16
            );

            self.framebuffer.line_vertical(
                Point::new(x as i32, 80 - (wall_height / 2)),
                *wall_height as u32,
            );
        }
    }
}

main_application! { MainApplication }

