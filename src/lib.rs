#![cfg_attr(not(test), no_std)]

/// control the vga printing
pub mod vga_buffer;

/// control the serial printing
pub mod serial;

/// shutdown by write `0` to x86 io port `0xf4`.
/// it works because of qemu device option `isa-debug-exit`.
pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}
