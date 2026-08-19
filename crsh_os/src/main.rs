#![no_std] //don't include std library
#![no_main] //disable standard entrypoint
#![feature(custom_test_frameworks)] //use custom test frameworks
#![test_runner(crate::test_runner)] //use test_runner crate for testing framework
#![reexport_test_harness_main = "testMain"] //rename test main fn

use core::panic::PanicInfo;

mod vga_buffer; //import custom vga buffer module

//called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lnprintr!("{}", info); //print panic info to VGA buffer
    
    loop {}
}

#[cfg(test)]
pub fn testRunner(tests: &[&dyn Fn()]) {
    lnprintr!("Running {} tests.", tests.len());
    for test in tests {
        test();
    }
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! { //custom entry point
    lnprintr!("Hello World{}", "!");

    #[cfg(test)]
    testMain();

    loop {}
}