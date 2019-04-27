//! basic boot test.
//!
//! all integration test example are started with name `test-` and
//! can be automatically run by `bootimage test`. every successful
//! test should print `ok` to serial port; failed tests should print
//! `failed` and optional message.
//!
#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_imports))]

use core::panic::PanicInfo;
use rust_os::{serial_println, exit_qemu};

#[cfg(not(test))]
#[panic_handler]
/// implementation of panic
fn panic(info: &PanicInfo) -> ! {
    // print `failed` due to test tool
    serial_println!("failed");
    serial_println!("{}", info);

    unsafe { exit_qemu() }
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
/// entry point
pub extern "C" fn _start() -> ! {
    // print `ok` due to test tool
    serial_println!("ok");

    unsafe { exit_qemu() }
    loop {}
}
