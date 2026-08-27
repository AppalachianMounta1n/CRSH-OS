#![no_std] //don't include std library
#![no_main] //disable standard entrypoint
#![feature(custom_test_frameworks)] //use custom test frameworks
#![test_runner(crsh_os::test_runner)] //use test_runner crate for testing framework
#![reexport_test_harness_main = "testMain"] //rename test main fn

use core::panic::PanicInfo;
use crsh_os::lnprintr;

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

    crash_os::init(); //initialize OS lib

    //invoke breakpoint exception
    x86_64::instructions::interrupts::int3();
    
    #[cfg(test)]
    testMain();

    lnprintr!("It didn't crash!");
    loop {}
}

#[test_case]
fn trivialAssertion() {
    assert_eq!(1, 1);
}