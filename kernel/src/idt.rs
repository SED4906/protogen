use core::panic;

use crate::pic::{pic_remap, pit_init};
use crate::{print, println};
use kernel_macros::import_isrs;
use x86_64::{
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
    VirtAddr,
};
use crate::framebuffer::{pixel, read_pixel};
use crate::keyboard::{keyboard_handler, keyboard_init};
use crate::mouse::{LAST_MOUSE_POS, mouse_handler, mouse_init, MOUSE_POS, UNDER_MOUSE};

static mut INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable = InterruptDescriptorTable::new();

#[no_mangle]
unsafe extern "sysv64" fn interrupt_handler(stack: *mut u64, error_code: u64, isr: u64) {
    let mut pushes_code = true;
    if error_code == 0xEA7BEEF {
        // this error doesn't push a code itself
        pushes_code = false;
    }
    match isr {
        44 => {
            let movement = mouse_handler();
            if let Some(movement) = movement {
                pixel(LAST_MOUSE_POS.0 as u64, LAST_MOUSE_POS.1 as u64, UNDER_MOUSE);
                UNDER_MOUSE = read_pixel(MOUSE_POS.0 as u64, MOUSE_POS.1 as u64);
                pixel(MOUSE_POS.0 as u64, MOUSE_POS.1 as u64, match movement {
                    (_, _, false, false, false) => 0xFF696969,
                    (_, _, true, false, false) => 0xFFFF6969,
                    (_, _, true, true, false) => 0xFFFFFF69,
                    (_, _, true, true, true) => 0xFFFFFFFF,
                    (_, _, false, true, true) => 0xFF69FFFF,
                    (_, _, false, false, true) => 0xFF6969FF,
                    (_, _, false, true, false) => 0xFF69FF69,
                    (_, _, true, false, true) => 0xFFFF69FF,
                });
            }
            return;
        }
        33 => {
            let key = keyboard_handler();
            if let Some(key) = key {
                if key.1 {
                    print!("{}", char::from(key.0));
                    if key.0 == 13 {
                        print!("{}", char::from(10));
                    }
                }
            }
            return;
        }
        _ => {
            println!(
                "...Exception #{:x} at {:x}, stack {:x}",
                isr,
                *stack,
                *stack.add(3)
            );
            if pushes_code {
                println!("Error code: {:x}", error_code);
            }
            panic!("Unhandled Exception");
        }
    }
}

import_isrs!();

pub unsafe fn build() {
    INTERRUPT_DESCRIPTOR_TABLE
        .divide_error
        .set_handler_addr(VirtAddr::new(isr0 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .debug
        .set_handler_addr(VirtAddr::new(isr1 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .non_maskable_interrupt
        .set_handler_addr(VirtAddr::new(isr2 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .breakpoint
        .set_handler_addr(VirtAddr::new(isr3 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .overflow
        .set_handler_addr(VirtAddr::new(isr4 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .bound_range_exceeded
        .set_handler_addr(VirtAddr::new(isr5 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .invalid_opcode
        .set_handler_addr(VirtAddr::new(isr6 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .device_not_available
        .set_handler_addr(VirtAddr::new(isr7 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .double_fault
        .set_handler_addr(VirtAddr::new(isr8 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .invalid_tss
        .set_handler_addr(VirtAddr::new(isr10 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .segment_not_present
        .set_handler_addr(VirtAddr::new(isr11 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .stack_segment_fault
        .set_handler_addr(VirtAddr::new(isr12 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .general_protection_fault
        .set_handler_addr(VirtAddr::new(isr13 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .page_fault
        .set_handler_addr(VirtAddr::new(isr14 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .x87_floating_point
        .set_handler_addr(VirtAddr::new(isr16 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .alignment_check
        .set_handler_addr(VirtAddr::new(isr17 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .machine_check
        .set_handler_addr(VirtAddr::new(isr18 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .simd_floating_point
        .set_handler_addr(VirtAddr::new(isr19 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .virtualization
        .set_handler_addr(VirtAddr::new(isr20 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .vmm_communication_exception
        .set_handler_addr(VirtAddr::new(isr29 as u64));
    INTERRUPT_DESCRIPTOR_TABLE
        .security_exception
        .set_handler_addr(VirtAddr::new(isr30 as u64));

    pic_remap();

    INTERRUPT_DESCRIPTOR_TABLE[32].set_handler_addr(VirtAddr::new(exit_task as u64));
    INTERRUPT_DESCRIPTOR_TABLE[33].set_handler_addr(VirtAddr::new(isr33 as u64));
    INTERRUPT_DESCRIPTOR_TABLE[44].set_handler_addr(VirtAddr::new(isr44 as u64));
    INTERRUPT_DESCRIPTOR_TABLE.load();

    pit_init();
    keyboard_init();
    mouse_init();
}

extern "sysv64" {
    fn exit_task();
}
