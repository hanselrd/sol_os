#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sol_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use sol_os::{hlt_loop, vga_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    vga_println!("[sol_os]");

    sol_os::init();

    // fn stack_overflow() {
    //     stack_overflow();
    // }

    // stack_overflow();

    #[cfg(test)]
    test_main();

    vga_println!("...");

    hlt_loop()
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    vga_println!("{info}");

    hlt_loop()
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sol_os::test_panic_handler(info)
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(1, 1);
}
