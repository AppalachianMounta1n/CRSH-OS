# CRSH-OS
Crsh OS is a Rust-based minimal OS. Development was performed using the following guide: https://os.phil-opp.com/freestanding-rust-binary/
Current Guide Page: https://os.phil-opp.com/testing/

## Bootloader

## Functionality
- To run the image in QEMU, run the following command: `qemu-system-x86_64 -drive format=raw,file=target/crsh_os-x86_64/debug/bootimage-crsh_os.bin`
- You can also use `cargo run` to build and run the boot image at the same time.
- Currently, CRSH_OS consists of a minimal kernel that boots into QEMU and allows the user to use the `printr` and `lnprintr` macros to write to the VGA buffer of the QEMU emulator.