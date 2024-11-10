#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod io;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    io::vga_buffer::print_something();

    loop {}
}

#[panic_handler]
fn panic_handler(_info: &PanicInfo) -> ! {
    loop {}
}