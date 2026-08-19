# CRSH-OS
Crsh OS is a Rust-based minimal OS. Development was performed using the following guide: https://os.phil-opp.com/freestanding-rust-binary/
Current Guide Page: https://os.phil-opp.com/cpu-exceptions/

## Bootloader

## Functionality
- To run the image in QEMU, run the following command: `qemu-system-x86_64 -drive format=raw,file=target/crsh_os-x86_64/debug/bootimage-crsh_os.bin`
- You can also use `cargo run` to build and run the boot image at the same time.
- Currently, CRSH_OS consists of a minimal kernel that boots into QEMU and allows the user to use the `printr` and `lnprintr` macros to write to the VGA buffer of the QEMU emulator.
- You can also use `serialPrintr` and `serialLnprintr` to write to serial instead. Testing and a basic `lib.rs` have both been implemented as well. Run the tests with `cargo test` but make sure to `cargo clean` between runs to ensure a clean testing environment.