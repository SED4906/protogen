use x86::io::{inb, outb};
use crate::framebuffer::get_framebuffer;

pub static mut MOUSE_POS: (u16, u16) = (0u16, 0u16);
pub static mut LAST_MOUSE_POS: (u16, u16) = (0u16, 0u16);
pub static mut UNDER_MOUSE: u32 = 0;
pub static mut MOUSE_CYCLE: u32 = 0;
pub static mut MOUSE_BYTES: (u8,u8,u8) = (0,0,0);

unsafe fn mouse_wait(a: bool) {
    if !a {
        while inb(0x64) & 1 == 0 {}
    } else {
        while inb(0x64) & 2 == 1 {}
    }
}

unsafe fn mouse_read() -> u8 {
    mouse_wait(false);
    return inb(0x60);
}

unsafe fn mouse_write(a: u8) {
    mouse_wait(true);
    outb(0x64, 0xD4);
    mouse_wait(true);
    outb(0x60, a);
}
pub unsafe fn mouse_init() {
    mouse_wait(true);
    outb(0x64, 0xA8);

    mouse_wait(true);
    outb(0x64, 0x20);
    mouse_wait(false);
    let status = inb(0x60) | 2;
    mouse_wait(true);
    outb(0x64, 0x60);
    mouse_wait(true);
    outb(0x60, (status | 2) & !32 );

    mouse_write(0xF6);
    let _ack = mouse_read();
    mouse_write(0xF4);
    let _ack = mouse_read();

    let a1 = inb(0x21);
    let a2 = inb(0xA1);
    outb(0x21, a1 & !4);
    outb(0xA1, a2 & !16);
}

pub unsafe fn mouse_handler() -> Option<(i16, i16, bool, bool, bool)> {
    let mut xrel = 0;
    let mut yrel = 0;
    match MOUSE_CYCLE {
        0 => MOUSE_BYTES.0 = mouse_read(),
        1 => MOUSE_BYTES.1 = mouse_read(),
        2 => MOUSE_BYTES.2 = mouse_read(),
        _ => {}
    };
    MOUSE_CYCLE += 1;
    if MOUSE_CYCLE >= 3 {
        let b0 = MOUSE_BYTES.0;
        let b1 = MOUSE_BYTES.1;
        let b2 = MOUSE_BYTES.2;
        let d = b1 as i16;
        xrel = d - (((b0 as i16) << 4) & 0x100);
        let d = b2 as i16;
        yrel = d - (((b0 as i16) << 3) & 0x100);
        LAST_MOUSE_POS = MOUSE_POS.clone();
        MOUSE_POS.0 = MOUSE_POS.0.saturating_add_signed(xrel);
        MOUSE_POS.1 = MOUSE_POS.1.saturating_add_signed(-yrel);
        let framebuffer_width = get_framebuffer().width;
        let framebuffer_height = get_framebuffer().height;
        if MOUSE_POS.0 >= framebuffer_width as u16 {
            MOUSE_POS.0 = (framebuffer_width - 1)  as u16;
        }
        if MOUSE_POS.1 >= framebuffer_height as u16 {
            MOUSE_POS.1 = (framebuffer_height - 1)  as u16;
        }
        MOUSE_CYCLE=0;
        outb(0xA0, 0x20);
        outb(0x20, 0x20);
        return Some((xrel,yrel, b0 & 1 == 1, b0 & 4 == 4, b0 & 2 == 2))
    }
    outb(0xA0, 0x20);
    outb(0x20, 0x20);
    None
}