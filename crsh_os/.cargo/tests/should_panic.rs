#![feature(custom_test_frameworks)]
#![test_runner(test_runner)]
#![reexport_test_harness_main = "testMain"]

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    testMain();

    loop {}
}

pub fn test_runner(tests: &[&dyn Fn()]) {
    serialLnprintr!("Running {} tests.", tests.len());
    for test in tests {
        test();
        serialLnprintr!("[Test did not panic.]");
        exitQemu(QemuExitCode::Failure);
    }
    exitQemu(QemuExitCode::Success);
}