#![no_std]
#![no_main]

extern crate bootloader_api;

use bootloader_api::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

/// os entry point
fn kernel_main(_boot_info: &'static mut BootInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello World!";

/// This function is called on panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}
