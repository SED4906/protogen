use core::panic;

use crate::println;
use kernel_macros::import_isrs;
use x86_64::{
    structures::idt::{InterruptDescriptorTable, InterruptStackFrame},
    VirtAddr,
};
static mut INTERRUPT_DESCRIPTOR_TABLE: InterruptDescriptorTable = InterruptDescriptorTable::new();

#[no_mangle]
unsafe extern "sysv64" fn interrupt_handler(stack: *mut u64, error_code: u64, isr: u64) {
    let mut pushes_code = true;
    if error_code == 0xEA7BEEF {
        // this error doesn't push a code itself
        pushes_code = false;
    }
    println!("...Exception #{:x} at {:x}", isr, *stack);
    if pushes_code {
        println!("Error code: {:x}", error_code);
    }
    match isr {
        _ => panic!("Unhandled Exception"),
    };
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
    INTERRUPT_DESCRIPTOR_TABLE.load();
}
