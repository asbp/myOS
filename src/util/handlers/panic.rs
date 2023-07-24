use core::panic::PanicInfo;

use crate::{println, util::{text_driver::{TEXT_DRIVER, TextDriverColor}, hlt_loop}};

pub fn panic(info: &PanicInfo) -> ! {
    println!("=========================================");
    TEXT_DRIVER.lock().set_color(TextDriverColor::Red, TextDriverColor::Black);
    println!("{}", info);
    TEXT_DRIVER.lock().reset_color();

    hlt_loop();
}