use core::fmt;
use spin::Mutex;
use vga::{
    colors::{Color16, TextModeColor},
    writers::{ScreenCharacter, Text80x25, TextWriter},
};
use x86_64::instructions::interrupts;

pub type TextDriverColor = Color16;

#[allow(dead_code)]
pub struct TextDriver {
    text_mode: Text80x25,
    x: usize,
    y: usize,
    w: usize,
    h: usize,
    background: TextDriverColor,
    foreground: TextDriverColor,
}

impl TextDriver {
    pub const fn new() -> TextDriver {
        TextDriver {
            text_mode: Text80x25::new(),
            x: 0,
            y: 0,
            w: 80,
            h: 25,
            foreground: TextDriverColor::White,
            background: TextDriverColor::Black,
        }
    }

    pub fn write_screen_char(&mut self, char: &ScreenCharacter) -> &mut Self {
        if char.get_character() == b'\n' {
            self.new_line();
        } else if char.get_character() == 0x08 {
            self.backspace();
            //self.write_screen_char(char);
            //self.new_line();
        } else {
            self.text_mode.write_character(self.x, self.y, *char);

            self.x += 1;

            if self.x >= self.w {
                self.new_line();
            }
        }

        if self.y >= self.h {
            self.scroll_up();
            self.y = self.h - 1;
        }

        self.text_mode
            .write_character(self.x, self.y, self.space_char());
        self.text_mode.set_cursor_position(self.x, self.y);

        return self;
    }

    pub fn write_byte_misc(
        &mut self,
        char: &u8,
        foreground: TextDriverColor,
        background: TextDriverColor,
    ) -> &mut Self {
        let screen_char = ScreenCharacter::new(*char, TextModeColor::new(foreground, background));

        return self.write_screen_char(&screen_char);
    }

    pub fn write_byte(&mut self, char: &u8) -> &mut Self {
        return self.write_byte_misc(char, self.foreground, self.background);
    }

    pub fn write_str_misc(
        &mut self,
        text: &str,
        foreground: TextDriverColor,
        background: TextDriverColor,
    ) -> &mut Self {
        for (_index, char) in text.bytes().enumerate() {
            self.write_byte_misc(&char, foreground, background);
        }

        return self;
    }

    pub fn write_str(&mut self, text: &str) -> &mut Self {
        return self.write_str_misc(text, self.foreground, self.background);
    }

    // Functions

    fn space_char(&self) -> ScreenCharacter {
        return ScreenCharacter::new(
            b' ',
            TextModeColor::new(TextDriverColor::White, TextDriverColor::Black),
        );
    }

    pub fn get_offset(self) -> usize {
        return self.w * self.y + self.x;
    }

    pub fn new_line(&mut self) -> &mut Self {
        self.x = 0;
        self.y += 1;

        return self;
    }

    pub fn backspace(&mut self) -> &mut Self {
        if self.x > 0 {
            self.x -= 1;
        }

        return self;
    }

    pub fn move_position(&mut self, x: usize, y: usize) {
        self.x = x;
        self.y = y;

        self.text_mode.set_cursor_position(self.x, self.y)
    }

    pub fn scroll_up(&mut self) {
        for y in 1..self.h {
            for x in 0..self.w {
                let screen_character = self.text_mode.read_character(x, y);

                self.text_mode.write_character(x, y - 1, screen_character)
            }
        }

        self.clear_row(self.h - 1);
    }

    pub fn clear_row(&mut self, y: usize) {
        for col in 0..self.w {
            self.text_mode.write_character(col, y, self.space_char());
        }
    }

    pub fn reset_color(&mut self) -> &mut Self {
        self.foreground = TextDriverColor::White;
        self.background = TextDriverColor::Black;

        return self;
    }

    pub fn set_color(
        &mut self,
        foreground: TextDriverColor,
        background: TextDriverColor,
    ) -> &mut Self {
        self.background = background;
        self.foreground = foreground;

        return self;
    }

    // Template
}

impl fmt::Write for TextDriver {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        self.write_str(s);
        Ok(())
    }
}

#[macro_export]
macro_rules! print {
    ($($arg:tt)*) => ($crate::util::text_driver::_print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! println {
    () => ($crate::print!("\n"));
    ($($arg:tt)*) => ($crate::print!("{}\n", format_args!($($arg)*)));
}

#[macro_export]
macro_rules! set_color {
    ($fg:item) => {
        $crate::util::text_driver::_set_color($fg, TextDriverColor::Black)
    };
}

#[macro_export]
macro_rules! reset_color {
    () => {
        $crate::util::text_driver::_reset_color()
    };
}

#[doc(hidden)]
pub fn _set_color(foreground: TextDriverColor, background: TextDriverColor) {
    TEXT_DRIVER.lock().set_color(foreground, background);
}

#[doc(hidden)]
pub fn _reset_color() {
    TEXT_DRIVER.lock().reset_color();
}

#[doc(hidden)]
pub fn _print(args: fmt::Arguments) {
    use core::fmt::Write;

    interrupts::without_interrupts(|| {
        // new
        TEXT_DRIVER.lock().write_fmt(args).unwrap();
    });
}

lazy_static::lazy_static! {
    pub static ref TEXT_DRIVER: Mutex<TextDriver> = Mutex::new(TextDriver::new());
}
