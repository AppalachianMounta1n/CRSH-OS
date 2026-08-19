#![no_std] //don't include std library
#![no_main] //disable standard entrypoint

use core::panic::PanicInfo;

mod vga_buffer; //import custom vga buffer module

//called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lnprintr!("{}", info); //print panic info to VGA buffer
    
    loop {}
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! { //custom entry point
    lnprintr!("Hello World{}", "!");
    panic!("UH OH SPAGHETTI-Os!");

    loop {}
}