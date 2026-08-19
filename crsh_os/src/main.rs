#![no_std] //don't include std library
#![no_main] //disable standard entrypoint

use core::panic::PanicInfo;

mod vga_buffer; //import custom vga buffer module

//called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! { //custom entry point
    vga_buffer::printr();

    loop {}
}