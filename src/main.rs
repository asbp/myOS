#![no_std]
#![no_main]
#![feature(abi_x86_interrupt)]
#![feature(box_into_inner)]

extern crate alloc;

use crate::{
    command::start_command_interpreter,
    util::{hlt_loop, init},
};

use bootloader::entry_point;

entry_point!(main);

pub mod util;

mod command;

#[panic_handler]
fn panic(info: &core::panic::PanicInfo) -> ! {
    util::handlers::panic::panic(info);
}

fn main(boot_info: &'static bootloader::BootInfo) -> ! {
    println!("myOS version 1.0");

    init(boot_info);

    println!("Now ready to receive command.");

    start_command_interpreter();

    hlt_loop();
}
