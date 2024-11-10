#![no_std]
#![no_main]

use core::panic::PanicInfo;

mod io;

#[no_mangle]
pub extern "C" fn _start() -> ! {
    println!("Hello world! Numbers: {}", 5);
    loop {}
}

#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    println!("SKIBIDIOS PANIC");
    println!("{}", info);
    loop {}
}