#![no_std]
#![no_main]

use core::panic::PanicInfo;
use sol_os::{hlt_loop, qemu, serial_print, serial_println};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    should_fail();
    serial_println!("[test did not panic]");

    qemu::exit(qemu::ExitCode::Failure);

    hlt_loop()
}

fn should_fail() {
    serial_print!("should_panic::should_fail...\t");
    assert_eq!(0, 1);
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    serial_println!("[ok]");

    qemu::exit(qemu::ExitCode::Success);

    hlt_loop()
}
