#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(sol_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;
use sol_os::{hlt_loop, vga_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    hlt_loop()
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    sol_os::test_panic_handler(info)
}

#[test_case]
fn test_vga_println() {
    vga_println!("test_vga_println output");
}
