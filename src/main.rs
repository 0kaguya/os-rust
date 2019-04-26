#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]

#![cfg_attr(test, allow(unused_imports))]
#![cfg_attr(test, allow(unused_attributes))]
#![allow(unreachable_code)]


mod vga_buffer;

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
    loop {}
}
