use x86_64::{structures::paging::{OffsetPageTable, PageTable, Mapper, PhysFrame, FrameAllocator, PageTableFlags, Page}, PhysAddr, VirtAddr};
use crate::{memory::{allocate, current_pagemap, MemoryError, Allocator}, println};
use elf_rs::{Elf,ElfFile, ProgramHeaderFlags};

pub struct Process<'a> {
    pub id: isize,
    pub pagemap: OffsetPageTable<'a>,
    pub registers: Registers,
}

pub struct Registers {
    pub rax: usize,pub rbx: usize,pub rcx: usize,pub rdx: usize,
    pub rsi: usize,pub rdi: usize,pub rsp: usize,pub rbp: usize,
    pub  r8: usize,pub  r9: usize,pub r10: usize,pub r11: usize,
    pub r12: usize,pub r13: usize,pub r14: usize,pub r15: usize,
}

pub unsafe fn create_new_pagemap<'a>() -> Result<OffsetPageTable<'a>,crate::memory::MemoryError> {
    let mut frame_addr = allocate::<PageTable>()?;
    (*frame_addr).zero();
    let mut kernel_pagemap = current_pagemap();
    for l4 in 256..512 {
        (*frame_addr)[l4] = kernel_pagemap.level_4_table()[l4].clone();
    }
    Ok(OffsetPageTable::new(&mut *(frame_addr as *mut _), crate::memory::hhdm()))
}

pub unsafe fn spawn<'a>(image: &[u8]) -> Result<Process<'a>, ProcessError> {
    let pagemap = create_new_pagemap()?;
    let current = current_pagemap();
    let elf = Elf::from_bytes(image)?;
    println!("{:?} header: {:?}", elf, elf.elf_header());

    for p in elf.program_header_iter() {
        println!("{:x?}", p);
        if p.ph_type() == 1 {
            for i in 0..p.memsz()/4096 {
                let mut flags = PageTableFlags::PRESENT|PageTableFlags::USER_ACCESSIBLE;
                if p.flags().contains(ProgramHeaderFlags::WRITE) {flags |= PageTableFlags::WRITABLE}
                let mut page = Allocator{}.allocate_frame().expect("Out of memory");
                let mut page_writing = &mut *(page.start_address().as_u64() as *mut [u8;4096]);
                let mut bytes_to_copy = 4096;
                let content_iter = p.content().iter().array_chunks::<4096>().skip(i);
                page_writing.copy_from_slice(content_iter);
                pagemap.map_to(Page::from_start_address(VirtAddr::new(p.vaddr() + i*4096)).expect("Address not aligned"), page, flags, &mut Allocator{});
            }
        }
    }

    for s in elf.section_header_iter() {
        println!("{:x?}", s);
    }

    let s = elf.lookup_section(b".text");
    println!("s {:?}", s);

    Ok(Process {id: 1, pagemap: pagemap, registers: Registers { rax: 0, rbx: 0, rcx: 0, rdx: 0, rsi: 0, rdi: 0, rsp: 0, rbp: 0, r8: 0, r9: 0, r10: 0, r11: 0, r12: 0, r13: 0, r14: 0, r15: 0 }})
}

#[derive(Debug)]
pub enum ProcessError {
    ElfParseError(elf_rs::Error),
    MemoryError(crate::memory::MemoryError)
}

impl core::fmt::Display for ProcessError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::ElfParseError(x) => f.write_fmt(format_args!("{:?}",x)),
            Self::MemoryError(x) => f.write_fmt(format_args!("{:?}",x))
        };
        Ok(())
    }
}

impl core::error::Error for ProcessError { }

impl From<elf_rs::Error> for ProcessError {
    fn from(value: elf_rs::Error) -> Self {
        Self::ElfParseError(value)
    }
}

impl From<MemoryError> for ProcessError {
    fn from(value: MemoryError) -> Self {
        Self::MemoryError(value)
    }
}