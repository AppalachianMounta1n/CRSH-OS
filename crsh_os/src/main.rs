#![no_std] //don't include std library
#![no_main] //disable standard entrypoint

use core::panic::PanicInfo;

//called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    loop {}
}