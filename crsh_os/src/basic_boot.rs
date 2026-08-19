#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "testMain"]

use core::panic::PanicInfo;
use crsh_os::lnprintr;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    testMain();

    loop {}
}

fn test_runner(tests: &[&dyn Fn()]) {
    unimplemented!();
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    loop {}
}

#![test_runner(crsh_os::test_runner)]

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crsh_os::testPanicHandler(info)
}

#[test-case]
fn testLnprintr() {
    lnprintr!("Test lnprintr output.");
}