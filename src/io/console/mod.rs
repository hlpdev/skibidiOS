use core::fmt;
use core::fmt::Write;
use crate::io::vga_buffer::WRITER;

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::io::console::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    WRITER.lock().write_fmt(args).unwrap();
}