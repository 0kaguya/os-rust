#![cfg_attr(not(test), no_std)]
#![cfg_attr(not(test), no_main)]
#![cfg_attr(test, allow(unused_import))]


use rust_os::{print, println};
use rust_os::serial_println;
use rust_os::exit_qemu;


#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    serial_println!("failed");

    unsafe { exit_qemu() }
    loop {}
}

#[cfg(not(test))]
#[no_mangle]
pub extern "C" fn _start() -> ! {
    print!("some word");
    println!(" and some other");
    println!("and additional");

    serial_println!("ok");
    unsafe { exit_qemu() }
    loop {}
}