#![no_std]
#![no_main]
#![feature(custom_test_frameworks)]
#![test_runner(s0ra_os::test_runner)]
#![reexport_test_harness_main = "test_main"]
#![allow(clippy::empty_loop)]

use core::panic::PanicInfo;
use s0ra_os::{
    println,
    vga_buffer::{BUFFER_HEIGHT, WRITER},
};

#[no_mangle]
pub extern "C" fn _start() -> ! {
    test_main();

    loop {}
}

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    s0ra_os::test_panic_handler(info)
}

#[test_case]
fn test_println_output() {
    let s = "Some test string that fits on a single line";
    println!("{}", s);
    for (i, c) in s.chars().enumerate() {
        let screen_char = WRITER.lock().buffer.chars[BUFFER_HEIGHT - 2][i].read();
        assert_eq!(char::from(screen_char.ascii_character), c);
    }
}

#[test_case]
fn trivial_assertion() {
    assert_eq!(0, 0);
}
