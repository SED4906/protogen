use x86_64::registers::control::Cr3;
use x86_64::{
    structures::paging::Mapper,
};

use crate::memory::allocate;
use crate::println;

pub struct Process<'a> {
    _pid: isize,
    _active: Option<u64>,
    pagemap: &'a mut [u64; 512],
    stack: usize,
    rip: usize,
    regs: [usize; 16],
    pub next: Option<*mut Process<'a>>,
    pub prev: Option<*mut Process<'a>>,
}

pub static mut CURRENT_PROCESS: Option<*mut Process> = None;
pub static mut KERNEL_PAGEMAP: Option<usize> = None;
pub static mut KERNEL_RIP: Option<usize> = None;

const PROCESS_ENTRY: usize = 0x2000;

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

pub unsafe fn store_kernel_pagemap() {
    KERNEL_PAGEMAP = Some(Cr3::read().0.start_address().as_u64() as usize);
}

pub unsafe fn create_process<'a>(image: &[u8]) -> Option<()> {
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
    let process_page = (allocate::<Process>().ok()? as u64 & 0x0000FFFFFFFFF000) as *mut Process;
    crate::memory::map_to(
        pagemap,
        0x1000,
        process_page as u64,
        7,
    )?;
    *process_page = Process {
        _pid: 1,
        _active: None,
        pagemap,
        stack: PROCESS_ENTRY + (image.len() & !0xFFF) + 0x7f00,
        regs: [0usize; 16],
        rip: PROCESS_ENTRY,
        next: None,
        prev: None,
    };
    if let Some(current_process) = &mut CURRENT_PROCESS {
        (*process_page).next = Some(*current_process);
        (*process_page).prev = Some(*((**current_process).prev.as_mut().unwrap()));
        (*(**current_process).prev.unwrap()).next = Some(process_page);
        (**current_process).prev = Some(process_page);
    } else {
        (*process_page).next = Some(process_page);
        (*process_page).prev = Some(process_page);
        CURRENT_PROCESS = Some(process_page)
    }
    println!("made process");
    Some(())
}

#[no_mangle]
unsafe extern "sysv64" fn restore_register(register: usize) -> usize {
    if let Some(process) = &CURRENT_PROCESS {
        (**process).regs[register]
    } else {
        panic!("No active process to restore register from");
    }
}

#[no_mangle]
unsafe extern "sysv64" fn store_register(register: usize, value: usize) {
    let process = 0x1000 as *mut Process;
    (*process).regs[register] = value
}

#[no_mangle]
unsafe extern "sysv64" fn restore_pagemap() -> usize {
    if let Some(process) = &mut CURRENT_PROCESS {
        core::ptr::from_mut((**process).pagemap) as usize
    } else {
        panic!("No active process to restore pagemap from");
    }
}

#[no_mangle]
unsafe extern "sysv64" fn restore_kernel_pagemap() -> usize {
    if let Some(kernel_pagemap) = &KERNEL_PAGEMAP {
        *kernel_pagemap
    } else {
        panic!("couldn't restore kernel pagemap");
    }
}

#[no_mangle]
unsafe extern "sysv64" fn restore_kernel_rip() -> usize {
    if let Some(kernel_rip) = &KERNEL_RIP {
        *kernel_rip
    } else {
        panic!("couldn't restore kernel instruction pointer");
    }
}

#[no_mangle]
unsafe extern "sysv64" fn store_kernel_rip(value: usize) {
    KERNEL_RIP = Some(value)
}

#[no_mangle]
unsafe extern "sysv64" fn restore_stack() -> usize {
    if let Some(process) = &CURRENT_PROCESS {
        (**process).stack
    } else {
        panic!("No active process to restore stack from");
    }
}

#[no_mangle]
unsafe extern "sysv64" fn store_stack(value: usize) {
    let process = 0x1000 as *mut Process;
    (*process).stack = value;
}

#[no_mangle]
unsafe extern "sysv64" fn restore_rip() -> usize {
    if let Some(process) = &CURRENT_PROCESS {
        (**process).rip
    } else {
        panic!("No active process to restore register from");
    }
}

#[no_mangle]
unsafe extern "sysv64" fn store_rip(value: usize) {
    let process = 0x1000 as *mut Process;
    (*process).rip = value;
}

extern "sysv64" {
    pub fn enter_task();
}
