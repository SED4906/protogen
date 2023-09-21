use x86::io::{inb, outb};

pub unsafe fn keyboard_init() {
    let a1 = inb(0x21);
    outb(0x21, a1 & !2);
}

const SCANCODES_KEYS: [u8;89] = *concat_bytes!([255, 0x1B], b"1234567890-=", [8, 9], b"qwertyuiop[]", [13, 255], b"asdfghjkl;'`", [255], b"\\zxcvbnm,./", [255], b'*', [255], b' ', [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255], b"789-456+1230.", [255, 255, 255, 255, 255]);
const SCANCODES_SHIFT_KEYS: [u8;89] = *concat_bytes!([255, 0x1B], b"!@#$%^&*()_+", [8, 9], b"QWERTYUIOP{}", [13, 255], b"ASDFGHJKL:\"~", [255], b"|ZXCVBNM<>?", [255], b'*', [255], b' ', [255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255, 255], b"789-456+1230.", [255, 255, 255, 255, 255]);

pub unsafe fn keyboard_handler() -> Option<(u8, bool)> {
    let scancode = inb(0x60);
    let key = match scancode {
        scan if scan < 89 => {
            if SCANCODES_KEYS[scan as usize] == 255 {
                None
            } else {
                Some((SCANCODES_KEYS[scan as usize], true))
            }
        }
        _ => None
    };
    outb(0x20, 0x20);
    key
}