#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]
#![feature(custom_test_frameworks)]
#![test_runner(s0ra_os::test_runner)]
#![reexport_test_harness_main = "test_main"]

use core::{arch::asm, fmt::Write, panic::PanicInfo, str::Chars};

use vga_buffer::{Color, WRITER};

mod serial;
mod vga_buffer;

const COMPUTER: &str = include_str!("../text/computer.txt");

#[no_mangle]
pub extern "C" fn _start() -> ! {
    #[cfg(test)]
    test_main();

    colored_print(COMPUTER.chars());

    // Update cursor position
    unsafe {
        asm!(include_str!("../asm/move_cursor.s"))
    }
    loop {}
}

fn colored_print(mut txt: Chars) {
    macro_rules! replace {
        ($color:tt) => {{
            $crate::vga_buffer::WRITER.lock().color_code.replace(
                $crate::vga_buffer::ColorCode::new(Color::$color, Color::Black),
            );
        }};
    }
    while let Some(ch) = txt.next() {
        if ch == '&' {
            match txt.next() {
                Some('0') => replace!(Black),
                Some('1') => replace!(Blue),
                Some('2') => replace!(Green),
                Some('3') => replace!(Cyan),
                Some('4') => replace!(Red),
                Some('5') => replace!(Magenta),
                Some('6') => replace!(Brown),
                Some('7') => replace!(LightGray),
                Some('8') => replace!(DarkGray),
                Some('9') => replace!(LightBlue),
                Some('a') => replace!(LightGreen),
                Some('b') => replace!(LightCyan),
                Some('c') => replace!(LightRed),
                Some('d') => replace!(Pink),
                Some('e') => replace!(Yellow),
                Some('f') => replace!(White),
                None => write!(WRITER.lock(), "&").unwrap(),
                Some(non_matching) => write!(WRITER.lock(), "&{non_matching}").unwrap(),
            }
        } else {
            write!(WRITER.lock(), "{ch}").unwrap();
        }
    }
}

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    printcln!(LightRed, "{}", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    s0ra_os::test_panic_handler(info)
}
