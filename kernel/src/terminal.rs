use crate::framebuffer;
use core::fmt;
use spin::Mutex;

struct Writer {}
static WRITER: Mutex<Writer> = Mutex::new(Writer {});
static mut COL: u64 = 0;
static mut ROW: u64 = 0;

impl fmt::Write for Writer {
    fn write_str(&mut self, s: &str) -> fmt::Result {
        for c in s.as_bytes() {
            unsafe {
                match c {
                    8 => {
                        COL = COL.saturating_sub(1);
                    }
                    9 => {
                        COL += 8;
                        if COL >= framebuffer::get_framebuffer().width / 8 {
                            COL = 0;
                            ROW += 1;
                            if ROW >= framebuffer::get_framebuffer().height / 16 {
                                ROW = 0;
                            }
                        }
                    }
                    13 => {
                        COL = 0;
                    }
                    10 => {
                        COL = 0;
                        ROW += 1;
                        if ROW >= framebuffer::get_framebuffer().height / 16 {
                            ROW = 0;
                        }
                    }
                    _ => {
                        framebuffer::rect(
                            COL * 8,
                            ROW * 16,
                            COL * 8 + 8,
                            ROW * 16 + 16,
                            0x00000000,
                            0x00000000,
                        );
                        framebuffer::character(COL * 8, ROW * 16, *c, 0xFFFFFFFF);
                        COL += 1;
                        if COL >= framebuffer::get_framebuffer().width / 8 {
                            COL = 0;
                            ROW += 1;
                            if ROW >= framebuffer::get_framebuffer().height / 16 {
                                ROW = 0;
                            }
                        }
                    }
                }
            }
        }

        Ok(())
    }
}

pub fn _print(args: fmt::Arguments) {
    // NOTE: Locking needs to happen around `print_fmt`, not `print_str`, as the former
    // will call the latter potentially multiple times per invocation.
    let mut writer = WRITER.lock();
    fmt::Write::write_fmt(&mut *writer, args).ok();
}

#[macro_export]
macro_rules! print {
    ($($t:tt)*) => { $crate::terminal::_print(format_args!($($t)*)) };
}

#[macro_export]
macro_rules! println {
    ()          => { $crate::print!("\n"); };
    // On nightly, `format_args_nl!` could also be used.
    ($($t:tt)*) => { $crate::print!("{}\n", format_args!($($t)*)) };
}
