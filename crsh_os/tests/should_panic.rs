#![no_std]
#![no_main]

use core::panic::PanicInfo;
use crsh_os::{exitQemu, serialPrintr, serialLnprintr, QemuExitCode};

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    shouldFail();
    serialLnprintr!("[Test did not panic.]");
    exitQemu(QemuExitCode::Failure);

    loop {}
}

fn shouldFail() {
    serialPprintr!("Should panic and fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serialLnprintr!("[OK]");
    exitQemu(QemuExitCode::Success);

    loop {}
}