use crate::{print, println};

use self::allocator::init_allocator;

pub mod allocator;
pub mod event_emitter;
pub mod gdt;
pub mod handlers;
pub mod idt;
pub mod intel8259;
pub mod memory;
pub mod text_driver;

extern crate alloc;

pub fn init(boot_info: &'static bootloader::BootInfo) {
    print!("Initializing allocator...");
    init_allocator(boot_info);
    println!(" OK.");

    print!("Initializing IDT...");
    idt::init_idt();
    println!(" OK.");

    print!("Initializing GDT...");
    gdt::init();
    println!(" OK.");

    unsafe {
        intel8259::PICS.lock().initialize();
    }
    x86_64::instructions::interrupts::enable();
}

pub fn hlt_loop() -> ! {
    loop {
        x86_64::instructions::hlt();
    }
}
