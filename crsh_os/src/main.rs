#![no_std]

use core::panic::PanicInfo;

//called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}