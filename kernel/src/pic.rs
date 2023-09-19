use crate::print;
use x86::io::{inb, outb};

pub unsafe fn pic_remap() {
    let a1 = inb(0x21);
    let a2 = inb(0xA1);
    outb(0x20, 0x11);
    outb(0xA0, 0x11);
    outb(0x21, 0x20);
    outb(0xA1, 0x28);
    outb(0x21, 4);
    outb(0xA1, 2);
    outb(0x21, 1);
    outb(0xA1, 1);
    outb(0x21, a1);
    outb(0xA1, a2);
}

pub unsafe fn pit_init() {
    outb(0x43, 0x34);
    outb(0x40, 0xFF);
    outb(0x40, 0xFF);
    let a1 = inb(0x21);
    let a2 = inb(0xA1);
    outb(0x21, a1 & !1);
    outb(0xA1, a2 & !1);
}

static mut TIMER_TICK: usize = 5;

#[no_mangle]
pub unsafe extern "sysv64" fn timer_tick() -> usize {
    if TIMER_TICK == 0 {
        TIMER_TICK = 5;
    }
    TIMER_TICK -= 1;
    outb(0x20, 0x20);
    print!("{TIMER_TICK}");
    TIMER_TICK
}
