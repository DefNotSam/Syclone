#![no_std]
#![no_main]

use bootloader::{entry_point, BootInfo};
use core::panic::PanicInfo;

entry_point!(kernel_main);

fn kernel_main(_boot_info: &'static BootInfo) -> ! {
    let message = b"Syclone OS booted successfully!";
    let buffer_ptr = 0xb8000 as *mut u8;

    for (i, &byte) in message.iter().enumerate() {
        unsafe {
            *buffer_ptr.offset((i * 2) as isize) = byte;
            *buffer_ptr.offset((i * 2 + 1) as isize) = 0x0f;
        }
    }

    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
