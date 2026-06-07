use core::fmt::{self, Write};

const BUFFER_ADDR: usize = 0xb8000;
const BUFFER_HEIGHT: usize = 25;
const BUFFER_WIDTH: usize = 80;

#[repr(transparent)]
struct Buffer {
    chars: [[VolatileScreenChar; BUFFER_WIDTH]; BUFFER_HEIGHT],
}

#[repr(C)]
#[derive(Clone, Copy)]
struct VolatileScreenChar {
    ascii_character: u8,
    color_code: u8,
}

pub struct Writer {
    column_position: usize,
    color_code: u8,
    buffer: &'static mut Buffer,
}

impl Writer {
    pub fn new() -> Writer {
        Writer {
            column_position: 0,
            color_code: 0x0f,
            buffer: unsafe { &mut *(BUFFER_ADDR as *mut Buffer) },
        }
    }

    pub fn write_byte(&mut self, byte: u8) {
        match byte {
            b'\n' => self.new_line(),
            byte => {
                if self.column_position >= BUFFER_WIDTH {
                    self.new_line();
                }

                let row = BUFFER_HEIGHT - 1;
                let col = self.column_position;
                self.buffer.chars[row][col] = VolatileScreenChar {
                    ascii_character: byte,
                    color_code: self.color_code,
                };
                self.column_position += 1;
            }
        }
    }

    fn new_line(&mut self) {
        for row in 1..BUFFER_HEIGHT {
            for col in 0..BUFFER_WIDTH {
                let character = self.buffer.chars[row][col];
                self.buffer.chars[row - 1][col] = character;
            }
        }
        self.clear_row(BUFFER_HEIGHT - 1);
        self.column_position = 0;
    }

    fn clear_row(&mut self, row: usize) {
        let blank = VolatileScreenChar { ascii_character: b' ', color_code: self.color_code };
        for col in 0..BUFFER_WIDTH {
            self.buffer.chars[row][col] = blank;
        }
    }
}

impl Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for byte in s.bytes() {
            self.write_byte(byte);
        }
        Ok(())
    }
}

pub fn print(args: fmt::Arguments) {
    let mut writer = Writer::new();
    writer.write_fmt(args).unwrap();
}

pub fn println(args: fmt::Arguments) {
    print(format_args!("{}\n", args));
}

#[macro_export]
macro_rules! vga_print {
    ($($arg:tt)*) => ($crate::vga_buffer::print(format_args!($($arg)*)));
}

#[macro_export]
macro_rules! vga_println {
    () => ($crate::vga_buffer::println(format_args!("")));
    ($fmt:expr) => ($crate::vga_buffer::println(format_args!($fmt)));
    ($fmt:expr, $($arg:tt)*) => ($crate::vga_buffer::println(format_args!($fmt, $($arg)*)));
}
