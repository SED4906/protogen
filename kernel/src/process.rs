use x86_64::registers::control::Cr3;
use x86_64::{
    structures::paging::Mapper,
};

use crate::memory::allocate;
use crate::{print, println};

pub struct Process<'a> {
    _pid: isize,
    _active: Option<u64>,
    pagemap: &'a mut [u64; 512],
    stack: usize,
    rip: usize,
    regs: [usize; 16],
}

pub static mut CURRENT_PROCESS: Option<&mut Process> = None;

const PROCESS_ENTRY: usize = 0x1000;

unsafe fn create_new_pagemap<'a>() -> Option<&'a mut [u64; 512]> {
    let mut pagemap = &mut *(allocate::<[u64; 512]>().ok()?);
    println!(
        "allocated {} as pagemap",
        core::ptr::from_mut(pagemap) as u64
    );
    pagemap.clone_from(&*(Cr3::read().0.start_address().as_u64() as *mut [u64; 512]));
    pagemap[0] = 0;
    Some(pagemap)
}

pub unsafe fn create_process<'a>(image: &[u8]) -> Option<&mut Process<'a>> {
    let pagemap = create_new_pagemap()?;
    let mut page_number = 0;
    for chunk in image.chunks(4096) {
        crate::memory::map_to(
            pagemap,
            PROCESS_ENTRY as u64 + 0x1000 * page_number,
            chunk.as_ptr() as u64 & 0x0000FFFFFFFFF000,
            7,
        )?;
        page_number += 1;
    }
    println!("mapped code");
    for _stack_page in 0..8 {
        crate::memory::map_to(
            pagemap,
            PROCESS_ENTRY as u64 + 0x1000 * page_number,
            allocate::<u8>().ok()? as u64 & 0x0000FFFFFFFFF000,
            7,
        )?;
        page_number += 1;
    }
    println!("mapped stack");
    let process_page = allocate::<Process>().ok()?;
    *process_page = Process {
        _pid: 1,
        _active: None,
        pagemap,
        stack: PROCESS_ENTRY + (image.len() & !0xFFF) + 0x7f00,
        regs: [0usize; 16],
        rip: PROCESS_ENTRY,
    };
    println!("made process");
    Some(&mut *process_page)
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
        core::ptr::from_mut(process.pagemap) as usize
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
