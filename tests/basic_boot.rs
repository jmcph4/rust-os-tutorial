#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(rust_os_tutorial::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::panic::PanicInfo;

use rust_os_tutorial::println;

#[no_mangle] // don't mangle the name of this function
pub extern "C" fn _start() -> ! {
    rust_os_tutorial::init();
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    rust_os_tutorial::test_panic_handler(info);
}

#[test_case]
fn test_println() {
    println!("test_println output");
}
