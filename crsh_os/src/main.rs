#![no_std] //don't include std library
#![no_main] //disable standard entrypoint
#![feature(custom_test_frameworks)] //use custom test frameworks
#![test_runner(crate::test_runner)] //use test_runner crate for testing framework
#![reexport_test_harness_main = "testMain"] //rename test main fn

use core::panic::PanicInfo;
use crsh_os::lnprintr;

mod vga_buffer; //import custom vga buffer module
mod serial; //import custom serial module

//called on panic
#[cfg(not(test))] //non-test panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lnprintr!("{}", info); //print panic info to VGA buffer
    
    loop {}
}

//test panic handler
#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crsh_os::testPanicHandler(info)
}

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! { //custom entry point
    lnprintr!("Hello World{}", "!");

    #[cfg(test)]
    testMain();
    
    loop {}
}

#[test_case]
fn trivialAssertion() {
    assert_eq!(1, 1);
}