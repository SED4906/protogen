use x86_64::{
    structures::paging::{
        Mapper, OffsetPageTable, Page, PageTable, PageTableFlags, PhysFrame, Size4KiB,
    },
    PhysAddr, VirtAddr,
};

use crate::memory::{allocate, current_page_map, hhdm, Allocator};

pub struct Process<'a> {
    _pid: isize,
    _active: Option<u64>,
    page_map: OffsetPageTable<'a>,
    stack: usize,
    rip: usize,
    regs: [usize; 15],
}

pub static mut CURRENT_PROCESS: Option<&mut Process> = None;
pub static mut PROCESSES: Option<Process> = None;

unsafe fn create_new_pagemap<'a>() -> Option<OffsetPageTable<'a>> {
    let mut current = current_page_map();
    let mut pagemap = OffsetPageTable::new(&mut *(allocate::<PageTable>().ok()?), hhdm());
    pagemap.level_4_table().zero();
    for entry in 256..512 {
        pagemap.level_4_table()[entry] = current.level_4_table()[entry].clone();
    }
    Some(pagemap)
}

pub unsafe fn create_process<'a>(image: &[u8]) -> Option<Process<'a>> {
    let mut pagemap = create_new_pagemap()?;
    let mut page_number = 0;
    for chunk in image.chunks(4096) {
        pagemap
            .map_to(
                Page::<Size4KiB>::from_start_address_unchecked(VirtAddr::new(
                    0x1000 + 0x1000 * page_number,
                )),
                current_page_map()
                    .translate_page(Page::<Size4KiB>::from_start_address_unchecked(
                        VirtAddr::new(chunk.as_ptr() as u64),
                    ))
                    .ok()?,
                PageTableFlags::PRESENT
                    | PageTableFlags::WRITABLE
                    | PageTableFlags::USER_ACCESSIBLE,
                &mut Allocator {},
            )
            .ok()?
            .ignore();
        page_number += 1;
    }
    for _stack_page in 0..8 {
        pagemap
            .map_to(
                Page::<Size4KiB>::from_start_address_unchecked(VirtAddr::new(
                    0x1000 + 0x1000 * page_number,
                )),
                PhysFrame::from_start_address_unchecked(PhysAddr::new(
                    allocate::<u8>().ok()? as u64
                )),
                PageTableFlags::PRESENT
                    | PageTableFlags::WRITABLE
                    | PageTableFlags::USER_ACCESSIBLE,
                &mut Allocator {},
            )
            .ok()?
            .ignore();
        page_number += 1;
    }

    Some(Process {
        _pid: 1,
        _active: None,
        page_map: pagemap,
        stack: 0x1000 + (image.len() & !0xFFF) + 0x7f00,
        regs: [0usize; 15],
        rip: 0x1000,
    })
}

#[no_mangle]
unsafe extern "sysv64" fn restore_register(register: usize) -> usize {
    if let Some(process) = &CURRENT_PROCESS {
        process.regs[register]
    } else {
        panic!("No active process to restore register from");
    }
}

#[no_mangle]
unsafe extern "sysv64" fn restore_pagemap() -> usize {
    if let Some(process) = &mut CURRENT_PROCESS {
        core::ptr::from_mut(process.page_map.level_4_table()) as usize
    } else {
        panic!("No active process to restore register from");
    }
}

#[no_mangle]
unsafe extern "sysv64" fn restore_stack() -> usize {
    if let Some(process) = &CURRENT_PROCESS {
        process.stack
    } else {
        panic!("No active process to restore register from");
    }
}

#[no_mangle]
unsafe extern "sysv64" fn restore_rip() -> usize {
    if let Some(process) = &CURRENT_PROCESS {
        process.rip
    } else {
        panic!("No active process to restore register from");
    }
}

extern "sysv64" {
    pub fn enter_task();
}
