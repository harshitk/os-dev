// #![feature(panic_handler)]
#![feature(panic_implementation)] // required for defining the panic handler
#![no_std] // don't link the Rust standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;

/// This function is called on panic.
#[panic_handler]
#[no_mangle]
pub fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

// On macOS:
//#[no_mangle]
/*
pub extern "C" fn main() -> ! {
    loop {}
}
*/
#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    // this function is the entry point, since the linker looks for a function
    // named `_start` by default
    loop {}
}

//#[no_mangle]
//pub extern "C" fn _start() -> ! {
//    loop {}
//}
