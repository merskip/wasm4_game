use core::{arch::wasm32, panic::PanicInfo};

mod system;
pub mod application;
pub mod framebuffer;
pub mod geometry;

#[allow(dead_code)]
pub fn trace(msg: &str) {
    unsafe {
        system::traceUtf8(msg.as_ptr(), msg.len());
    }
}

#[panic_handler]
fn panic_handler(_panic_info: &PanicInfo<'_>) -> ! {
    wasm32::unreachable();
}
