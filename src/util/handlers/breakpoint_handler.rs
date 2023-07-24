use x86_64::structures::idt::InterruptStackFrame;

use crate::{util::text_driver::{TEXT_DRIVER, TextDriverColor}, println};

pub extern "x86-interrupt" fn breakpoint_handler(
    stack_frame: InterruptStackFrame)
{
    println!("=========================================");
    TEXT_DRIVER.lock().set_color(TextDriverColor::Red, TextDriverColor::Black);
    println!("EXCEPTION: BREAKPOINT\n{:#?}", stack_frame);
    TEXT_DRIVER.lock().reset_color();
}