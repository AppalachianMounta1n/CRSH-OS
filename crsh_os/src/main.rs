#![no_std] //don't include std library
#![no_main] //disable standard entrypoint
#![feature(custom_test_frameworks)] //use custom test frameworks
#![test_runner(crate::test_runner)] //use test_runner crate for testing framework
#![reexport_test_harness_main = "testMain"] //rename test main fn

use core::panic::PanicInfo;

mod vga_buffer; //import custom vga buffer module
mod serial; //import custom serial module

//called on panic
#[cfg(not(test))] //non-test panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    lnprintr!("{}", info); //print panic info to VGA buffer
    
    loop {}
}

//called on test panic
#[cfg(test)] //non-test panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serialLnprintr!("[Failed]\n");
    serialLnprintr!("Error: {}\n", info);
    exitQemu(QemuExitCode::Failed);
    
    loop {}
}

//trait to make auto-printing tests easier
pub trait Testable {
    fn run(&self) -> ();
}

impl<T> Testable for T where T: Fn(), {
    fn run(&self) {
        serialPrintr!("{}...\t", core::any::type_name::<T>());
        self();
        serialLnprintr!("[OK]");
    }
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serialLnprintr!("Running {} tests.", tests.len());
    for test in tests {
        test.run();
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
    assert_eq!(1, 1);
}