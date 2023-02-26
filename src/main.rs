#![no_main]
#![no_std]

extern crate alloc;

use alloc::format;
use alloc::string::ToString;
use wasm4 as w4;

struct Runtime {
    count: i32,
    framebuffer: w4::draw::Framebuffer,
}

impl w4::rt::Runtime for Runtime {
    fn start(resources: w4::rt::Resources) -> Self {
        Runtime {
            count: 0,
            framebuffer: resources.framebuffer
        }
    }

    fn update(&mut self) {
        if self.count % 60 == 0 {
            w4::trace("tick");
            self.count = 0;
        }
        self.count += 1;

        self.framebuffer.text("Hello world", [10, 10]);
    }
}

w4::main! { Runtime }