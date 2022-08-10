#![no_std]
#![no_main]
#![allow(clippy::empty_loop)]

use core::{arch::asm, fmt::Write, panic::PanicInfo, str::Chars};

use vga_buffer::{Color, WRITER};

mod vga_buffer;

const COMPUTER: &str = include_str!("../text/computer.txt");

#[no_mangle]
pub extern "C" fn _start() -> ! {
    colored_print(COMPUTER.chars());

    // Update cursor position
    unsafe {
        asm!(
            "
            mov dl, 80
            mov bx, 21
            mov ax, 9
            mul dl
            add bx, ax

            mov dx, 0x03D4
            mov al, 0x0F
            out dx, al

            inc dl
            mov al, bl
            out dx, al

            dec dl
            mov al, 0x0E
            out dx, al

            inc dl
            mov al, bh
            out dx, al
        "
        );
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

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    printcln!(LightRed, "{}", info);
    loop {}
}
