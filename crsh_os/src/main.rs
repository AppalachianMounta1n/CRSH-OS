#![no_std] //don't include std library
#![no_main] //disable standard entrypoint

use core::panic::PanicInfo;

//called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! { //custom entry point
    let vga_buffer = 0xb8000 as *mut u8; //create VGA buffer at address 0xb8000

    for (i, &byte) in HELLO.iter().enumerate() { //iterate through the HELLO variable for bytes
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte; //write byte
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb; //set byte color
        }
    }

    loop {}
}