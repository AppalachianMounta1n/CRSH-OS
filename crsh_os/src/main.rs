#![no_std] //don't include std library
#![no_main] //disable standard entrypoint
#![feature(custom_test_frameworks)] //use custom test frameworks
#![test_runner(crate::test_runner)] //use test_runner crate for testing framework
#![reexport_test_harness_main = "testMain"] //rename test main fn

use core::panic::PanicInfo;

mod vga_buffer; //import custom vga buffer module
mod serial; //import custom serial module

//called on panic
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lnprintr!("{}", info); //print panic info to VGA buffer
    
    loop {}
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Fn()]) {
    lnprintr!("Running {} tests.", tests.len());
    for test in tests {
        test();
    }

    exitQemu(QemuExitCode::Success); //exit Qemu after running tests
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failure = 0x11,
}

pub fn exitQemu(exitCode: QemuExitCode) { //exit Qemu with custom exit codes
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exitCode as u32);
    }
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
    printr!("Trivial assertion... ");
    assert_eq!(1, 1);
    lnprintr!("[OK]");
}