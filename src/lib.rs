
#![feature(lang_items, const_fn, unique, const_unique_new)]
#![no_std]

#[macro_use]
mod vga_buffer;
mod memory;

extern crate rlibc;
extern crate volatile;
extern crate spin;
extern crate multiboot2;


#[no_mangle]
pub extern fn rust_main(multiboot_information_address: usize) {

	vga_buffer::clear_screen();

	let boot_info = unsafe { multiboot2::load(multiboot_information_address) };
	let memory_map_tag = boot_info.memory_map_tag()
		.expect("Memory Tag Required");
	println!("memory Areas");
	for area in memory_map_tag.memory_areas() {
		println!("    start: 0x{:x}, length: 0x{:x}",
		   area.base_addr, area.length);
	}

	let elf_sections_tag = boot_info.elf_sections_tag()
    .expect("Elf-sections tag required");

	println!("kernel sections:");
	for section in elf_sections_tag.sections() {
	    println!("    addr: 0x{:x}, size: 0x{:x}, flags: 0x{:x}",
	        section.addr, section.size, section.flags);
	}

	let kernel_start = elf_sections_tag.sections().map( |s| s.addr)
		.min().unwrap();
	let kernel_end = elf_sections_tag.sections().map(|s| s.addr)
		.max().unwrap();
	let multiboot_start = multiboot_information_address;
	let multiboot_end 	= multiboot_start + (boot_info.total_size as usize);

	println!("kernel start: 0x{:x} kernel end: 0x{:x}, size: {} kbytes", kernel_start, kernel_end, (kernel_end - kernel_start)/1000);
	println!("multiboot start: 0x{:x} multiboot end: 0x{:x}, size: {} bytes", multiboot_start, multiboot_end, multiboot_end - multiboot_start);

	let mut frame_allocator = memory::AreaFrameAllocator::new(
	    kernel_start as usize, kernel_end as usize, multiboot_start,
	    multiboot_end, memory_map_tag.memory_areas());
		
		

}

#[lang = "eh_personality"] extern fn eh_personality() {}
#[lang = "panic_fmt"] 
#[no_mangle] 
pub extern fn panic_fmt(fmt: core::fmt::Arguments, file: &'static str, line: u32) -> ! {
	println!("\n\nPANIC in {} at line {}", file, line);
	println!("    {}", fmt);
	loop{}
}

