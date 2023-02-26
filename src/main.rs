#![no_main]
#![no_std]

mod world_map;
mod player_state;

use core::{arch::wasm32, panic::PanicInfo};
use crate::player_state::PlayerState;
use crate::world_map::WorldMap;

extern "C" {
    fn vline(x: i32, y: i32, len: u32);
}

const GAMEPAD1: *const u8 = 0x16 as *const u8;

const BUTTON_LEFT: u8 = 16;  // 00010000
const BUTTON_RIGHT: u8 = 32; // 00100000
const BUTTON_UP: u8 = 64;    // 01000000
const BUTTON_DOWN: u8 = 128; // 10000000

struct Runtime {
    world_map: WorldMap,
    player_state: PlayerState,
}

impl Runtime {
    fn new() -> Self {
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
        }
    }

    fn update(&mut self) {
        self.player_state.update(
            &self.world_map,
            unsafe { *GAMEPAD1 & BUTTON_UP != 0 },
            unsafe { *GAMEPAD1 & BUTTON_DOWN != 0 },
            unsafe { *GAMEPAD1 & BUTTON_LEFT != 0 },
            unsafe { *GAMEPAD1 & BUTTON_RIGHT != 0 },
        );
    }
}

#[panic_handler]
fn phandler(_: &PanicInfo<'_>) -> ! {
    wasm32::unreachable();
}

#[no_mangle]
unsafe fn update() {
    vline(80, 20, 120);
}
