#![no_main]
#![no_std]

mod wasm4;
mod renderable;
mod world_map;
mod world_map_render;
mod player_state;
mod camera;
mod ray_casting;

use core::f32::consts::PI;
use crate::camera::Camera;
use crate::player_state::PlayerState;
use crate::renderable::Renderable;
use crate::wasm4::application::*;
use crate::wasm4::framebuffer::Framebuffer;
use crate::wasm4::gamepad::Gamepad;
use crate::wasm4::geometry::{Point, Size};
use crate::world_map::WorldMap;
use crate::world_map_render::WorldMapRender;

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
        let camera = Camera::new(Point::new(1.5, 1.5), 30.0, PI / 2.7);
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
            player_state: PlayerState::new(camera, 0.045),
            gamepad: Gamepad::gamepad1(),
            framebuffer,
        }
    }

    fn update(&mut self) {
        self.player_state.update_movement(&self.world_map, &self.gamepad);

        self.framebuffer.set_color(Color::Ground as u16);
        self.framebuffer.rectangle(Point::new(0, 80), Size::new(160, 160));

        let world_map_render = WorldMapRender::new(&self.world_map, &self.player_state.camera);
        world_map_render.render(&self.framebuffer);
    }
}

main_application! { MainApplication }

