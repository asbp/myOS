use alloc::string::{String, ToString};
use lazy_static::lazy_static;
use pc_keyboard::{layouts, DecodedKey, HandleControl, Keyboard, ScancodeSet1};
use spin::Mutex;
use x86_64::structures::idt::InterruptStackFrame;

use crate::util::{
    event_emitter::EVENT_EMITTER,
    intel8259::{InterruptIndex, PICS}, task::keyboard::add_scancode,
};

pub extern "x86-interrupt" fn keyboard_interrupt_handler(_stack_frame: InterruptStackFrame) {
    use x86_64::instructions::port::Port;

    lazy_static! {
        static ref KEYBOARD: Mutex<Keyboard<layouts::Us104Key, ScancodeSet1>> =
            Mutex::new(Keyboard::new(
                ScancodeSet1::new(),
                layouts::Us104Key,
                HandleControl::Ignore
            ));
    }

    let mut keyboard = KEYBOARD.lock();

    let mut port = Port::new(0x60);
    let scancode: u8 = unsafe { port.read() };

    EVENT_EMITTER
        .lock()
        .emit("keyboard_scancode", &[&String::from(scancode.to_string())]);

    if let Ok(Some(key_event)) = keyboard.add_byte(scancode) {
        if let Some(key) = keyboard.process_keyevent(key_event) {
            match key {
                DecodedKey::Unicode(character) => emit_unicode_key(&character),
                DecodedKey::RawKey(key) => emit_raw_key(&String::from(alloc::format!("{:?}", key))),
            }
        }
    }

    unsafe {
        PICS.lock()
            .notify_end_of_interrupt(InterruptIndex::Keyboard.as_u8());
    }
}

fn emit_unicode_key(char: &char) {
    EVENT_EMITTER
        .lock()
        .emit("keyboard_unicode_key", &[&String::from(char.to_string())]);

    add_scancode(String::from(*char));
}

fn emit_raw_key(code_str: &str) {
    EVENT_EMITTER.lock().emit("keyboard_raw_key", &[code_str]);

    add_scancode(code_str.to_string());
}