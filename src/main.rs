// src/main.rs
#![no_std]  // no standard library
#![no_main] // disable all Rust-level entry points

use core::panic::PanicInfo;
use core::fmt::Write;

// this function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

mod vga_buffer;
static HELLO: &[u8] = b"Hello World!";

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    vga_buffer::print_something();
    loop {}
}
