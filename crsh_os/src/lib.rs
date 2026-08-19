#![no_std]
#![cfg_attr(test, no_main)]
#![feature(custom_test_frameworks)]
#![test_runner(crate::test_runner)]
#![reexport_test_harness_main = "testMain"]

use core::panic::PanicInfo;

pub mod vga_buffer; //import custom vga buffer module
pub mod serial; //import custom serial module

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

pub fn testPanicHandler(info: &PanicInfo) -> ! {
    serialLnprintr!("[failed]");
    serialLnprintr!("Error: {}", info);
    exitQemu(QemuExitCode::Failure);
    loop {}
}

//called on test panic
#[cfg(test)] //non-test panic handler
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    testPanicHandler(info)
}

#[cfg(test)]
pub fn test_runner(tests: &[&dyn Testable]) {
    serialLnprintr!("Running {} tests.", tests.len());
    for test in tests {
        test.run();
    }

    exitQemu(QemuExitCode::Success); //exit Qemu after running tests
}

//entrypoint for cargo test
#[cfg(test)]
#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    testMain();
    
    loop {}
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