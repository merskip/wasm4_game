use core::ops::BitAnd;
use crate::wasm4::system;

pub struct Gamepad {
    address: *const u8
}

#[repr(u8)]
#[allow(dead_code)]
pub enum GamepadButton {
    ButtonX = system::BUTTON_1,
    ButtonY = system::BUTTON_2,
    DPadLeft = system::BUTTON_LEFT,
    DPadRight = system::BUTTON_RIGHT,
    DPadUp = system::BUTTON_UP,
    DPadDown = system::BUTTON_DOWN,
}

#[allow(dead_code)]
impl Gamepad {
    unsafe fn new(address: *const u8) -> Self {
        Gamepad { address }
    }

    pub fn gamepad1() -> Self {
        unsafe { Gamepad::new(system::GAMEPAD1) }
    }

    pub fn gamepad2() -> Self {
        unsafe { Gamepad::new(system::GAMEPAD2) }
    }

    pub fn gamepad3() -> Self {
        unsafe { Gamepad::new(system::GAMEPAD3) }
    }

    pub fn gamepad4() -> Self {
        unsafe { Gamepad::new(system::GAMEPAD4) }
    }

    pub fn is_pressed(&self, button: GamepadButton) -> bool {
        unsafe {
            (*self.address).bitand(button as u8) != 0
        }
    }
}