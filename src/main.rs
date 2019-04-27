#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#![cfg_attr(test, allow(unused_imports))]
#![cfg_attr(test, allow(unused_attributes))]
#![allow(unreachable_code)]


mod vga_buffer;
mod serial;

use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello Rust");
    serial_println!("From Guest");
    loop {}
}

/// shutdown by write `0` to x86 io port `0xf4`.
/// it works because of qemu device option `isa-debug-exit`.
pub unsafe fn exit_qemu() {
    use x86_64::instructions::port::Port;

    let mut port = Port::<u32>::new(0xf4);
    port.write(0);
}
