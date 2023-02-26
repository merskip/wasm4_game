use crate::wasm4::framebuffer::Framebuffer;

pub trait Application {
    /// Called at the start of the game, before the first update.
    fn start(framebuffer: Framebuffer) -> Self;

    /// Called every frame, about 60 times per second.
    fn update(&mut self);
}

#[macro_export]
macro_rules! main_application {
    ($application:ty) => {
        static mut MAIN_APPLICATION: core::mem::MaybeUninit<$application> = core::mem::MaybeUninit::uninit();

        #[no_mangle]
        unsafe extern "C" fn start() {
            let framebuffer = $crate::wasm4::framebuffer::Framebuffer::new();
            let application = <$application as $crate::wasm4::application::Application>::start(framebuffer);
            unsafe {
                MAIN_APPLICATION = core::mem::MaybeUninit::new(application);
            }
        }

        #[no_mangle]
        unsafe extern "C" fn update() {
            let application = unsafe { MAIN_APPLICATION.assume_init_mut() };
            <$application as $crate::wasm4::application::Application>::update(application);
        }
    };
}