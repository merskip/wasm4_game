#![no_main]
#![no_std]

mod world_map;
mod player_state;
mod wasm4;

use crate::player_state::PlayerState;
use crate::wasm4::application::*;
use crate::wasm4::framebuffer::Framebuffer;
use crate::wasm4::gamepad::Gamepad;
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
        self.player_state.update(&self.world_map, &self.gamepad);
    }
}

main_application! { MainApplication }

