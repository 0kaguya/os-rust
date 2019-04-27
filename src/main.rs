#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use rust_os::println;

#[cfg(not(test))]
#[panic_handler]
/// implementation of panic
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
/// entry point
pub extern "C" fn _start() -> ! {
    println!("Hello Rust");
    loop {}
}