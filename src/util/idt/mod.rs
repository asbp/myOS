use x86_64::structures::idt::InterruptDescriptorTable;

use crate::util::{gdt, intel8259::InterruptIndex};

use super::handlers::*;

use lazy_static::lazy_static;

lazy_static! {
    static ref IDT: InterruptDescriptorTable = {
        let mut idt = InterruptDescriptorTable::new();

        idt.breakpoint
            .set_handler_fn(breakpoint_handler::breakpoint_handler);
            unsafe {
                idt.double_fault.set_handler_fn(dbl_fault::double_fault_handler)
                    .set_stack_index(gdt::DOUBLE_FAULT_IST_INDEX); // new
            }

        idt.page_fault.set_handler_fn(page_fault::page_fault_handler);

        idt[InterruptIndex::Timer.as_usize()]
            .set_handler_fn(intel8253::timer_interrupt_handler); // new
        idt[InterruptIndex::Keyboard.as_usize()]
            .set_handler_fn(keyboard::keyboard_interrupt_handler); // new
        idt
    };
}

pub fn init_idt() {
    IDT.load();
}
