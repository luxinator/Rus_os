#![feature(lang_items, const_fn, unique)]
#![no_std]
extern crate rlibc;
extern crate volatile;
extern crate spin;

mod vga_buffer;



#[no_mangle]
pub extern fn rust_main() {
	use core::fmt::Write;
	vga_buffer::WRITER.lock().write_str("Hello Again!");
	write!(vga_buffer::WRITER.lock(), ", some numbers: \n\n\n\n{} {}", 42, 3.1415);
	loop {}
}

#[allow(non_snake_case)]
#[no_mangle]
pub extern "C" fn _Unwind_Resume() -> ! {
	loop {}
}


#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] extern fn panic_fmt() -> ! {loop{}}
