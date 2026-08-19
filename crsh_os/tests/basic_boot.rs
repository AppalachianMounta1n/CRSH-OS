#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(crsh_os::test_runner)]
#![reexport_test_harness_main = "testMain"]

use core::panic::PanicInfo;
use crsh_os::lnprintr;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    testMain();
    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    crsh_os::testPanicHandler(info)
}

#[test_case]
fn testLnprintr() {
    lnprintr!("Test lnprintr output.");
}