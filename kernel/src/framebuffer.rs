use core::usize;

use limine::{NonNullPtr, Framebuffer};

static FRAMEBUFFER_REQUEST: limine::FramebufferRequest = limine::FramebufferRequest::new(0);

pub fn get_framebuffer() -> &'static NonNullPtr<Framebuffer> {
    if let Some(framebuffer_response) = FRAMEBUFFER_REQUEST.get_response().get() {
        if framebuffer_response.framebuffer_count < 1 {
            crate::hcf()
        }
        // Get the first framebuffer's information.
        &framebuffer_response.framebuffers()[0]
    } else {
        crate::hcf()
    }
}

pub fn pixel(x: u64, y: u64, color: u32) {
    let framebuffer = get_framebuffer();
    let pixel_offset = y as usize * framebuffer.pitch as usize + x as usize * 4;
    unsafe{*(framebuffer.address.as_ptr().unwrap().add(pixel_offset) as *mut u32) = color;}
}

pub fn rect(x0: u64, y0: u64, x1: u64, y1: u64, border: u32, fill: u32) {
    for py in y0..=y1 {
        for px in x0..=x1 {
            if px == x0 || px == x1 || py == y0 || py == y1 {
                pixel(px, py, border)
            } else {
                pixel(px, py, fill)
            }
        }
    }
}

#[allow(dead_code)]
pub fn line(x0: u64, y0: u64, x1: u64, y1: u64, color: u32) {
    let mut x0: i64 = x0.try_into().unwrap();
    let mut y0: i64 = y0.try_into().unwrap();
    let x1: i64 = x1.try_into().unwrap();
    let y1: i64 = y1.try_into().unwrap();
    let dx: i64 = if x1>x0 {x1-x0} else {x0-x1};
    let sx: i64 = if x0 < x1 {1} else {-1};
    let dy: i64 = if y1>y0 {y0-y1} else {y1-y0};
    let sy: i64 = if y0 < y1 {1} else {-1};
    let mut err = dx + dy;
    loop {
        pixel(x0.unsigned_abs(),y0.unsigned_abs(),color);
        if x0 == x1 && y0 == y1 {break;}
        let e2 = 2*err;
        if e2 >= dy {
            if x0 == x1 {break}
            err += dy;
            x0 += sx;
        }
        if e2 <= dx {
            if y0 == y1 {break}
            err += dx;
            y0 += sy;
        }
    }
}

pub fn clear(color: u32) {
    let framebuffer_width = get_framebuffer().width;
    let framebuffer_height = get_framebuffer().height;
    for y in 0..framebuffer_height {
        for x in 0..framebuffer_width {
            pixel(x, y, color)
        }
    }
}

static FONT: &[u8] = include_bytes!("unifont.bin");

pub fn character(x: u64, y: u64, c: u8, color: u32) {
    for py in y..y+16 {
        for px in x..x+8 {
            if FONT[(c as usize) * 16 + py as usize - y as usize] & (128 >> (px-x)) != 0 {
                pixel(px, py, color);
            }
        }
    }
}

/*pub fn string(x: u64, y: u64, s: &[u8], wrap: Option<u64>, color: u32) {
    let mut line_length = 0;
    let mut line = 0;
    for c in s {
        match c {
            8 => line_length -= 1,
            9 => line_length += 8,
            13 => line_length = 0,
            10 => {line_length = 0;line+=1;},
            _ => {character(x + line_length * 8, y + line * 16, *c, color);line_length += 1;}
        };
        if let Some(wrap) = wrap {
            if line_length >= wrap {
                line_length = 0;
                line += 1;
            }
        }
    }
}*/