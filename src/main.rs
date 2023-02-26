#![no_main]
#![no_std]

mod world_map;

extern crate alloc;

use wasm4 as w4;
use crate::world_map::WorldMap;

struct Runtime {
    world_map: WorldMap,
    framebuffer: w4::draw::Framebuffer,
}

impl w4::rt::Runtime for Runtime {
    fn start(resources: w4::rt::Resources) -> Self {
        Runtime {
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
            framebuffer: resources.framebuffer,
        }
    }

    fn update(&mut self) {
        self.framebuffer.text("Hello world", [10, 10]);
    }
}

w4::main! { Runtime }