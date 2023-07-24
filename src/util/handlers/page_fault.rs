use x86_64::structures::idt::{InterruptStackFrame, PageFaultErrorCode};

use crate::{println, util::{hlt_loop, text_driver::{TEXT_DRIVER, TextDriverColor}}};

pub extern "x86-interrupt" fn page_fault_handler(
    stack_frame: InterruptStackFrame,
    error_code: PageFaultErrorCode,
) {
    use x86_64::registers::control::Cr2;

    TEXT_DRIVER.lock().set_color(TextDriverColor::Red, TextDriverColor::Black);
    println!("EXCEPTION: PAGE FAULT");
    println!("Accessed Address: {:?}", Cr2::read());
    println!("Error Code: {:?}", error_code);
    println!("{:#?}", stack_frame);
    
    hlt_loop();
}