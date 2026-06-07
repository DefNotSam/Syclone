use crate::vga_buffer::{vga_println};

pub fn run() -> ! {
    vga_println!("Syclone OS: kernel core initialized");
    vga_println!("Initializing subsystems...");

    crate::fs::init();
    crate::net::init();
    crate::compat::init();

    vga_println!("Subsystems initialized.");
    vga_println!("Welcome to Syclone. More features coming soon.");

    loop {}
}
