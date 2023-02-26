use core::{arch::wasm32, panic::PanicInfo};

mod system;
pub mod application;

pub fn trace(msg: &str) {
    unsafe {
        system::traceUtf8(msg.as_ptr(), msg.len());
    }
}

#[panic_handler]
fn panic(_panic_info: &PanicInfo<'_>) -> ! {
    wasm32::unreachable();
}
