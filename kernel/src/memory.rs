struct Freelist(*mut Freelist);
static mut FREELIST: Freelist = Freelist(core::ptr::null_mut());
static MEMMAP_REQUEST: limine::MemmapRequest = limine::MemmapRequest::new(0);
static HHDM_REQUEST: limine::HhdmRequest = limine::HhdmRequest::new(0);

use x86_64::{
    registers::control::Cr3,
    structures::paging::{FrameAllocator, OffsetPageTable, PhysFrame, Size4KiB},
    PhysAddr, VirtAddr,
};

static mut HHDM: Option<VirtAddr> = None;

#[derive(Debug)]
pub enum MemoryError {
    OutOfMemory,
}
impl core::error::Error for MemoryError {}
impl core::fmt::Display for MemoryError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            &Self::OutOfMemory => f.write_str("Out of memory")?,
        };
        Ok(())
    }
}

pub struct Allocator {}

pub fn allocate<T>() -> Result<*mut T, MemoryError> {
    unsafe {
        if FREELIST.0.is_null() {
            return Err(MemoryError::OutOfMemory);
        }
        let next = &mut *FREELIST.0;
        let current = FREELIST.0;
        FREELIST.0 = next;
        Ok(current as *mut T)
    }
}

pub fn free<T>(page: *mut T) {
    let quantized_page = (page as usize & !0xFFF) as *mut Freelist;
    unsafe {
        (*quantized_page).0 = FREELIST.0;
        FREELIST.0 = quantized_page;
    }
}

pub fn build() {
    if let Some(memory_map_response) = MEMMAP_REQUEST.get_response().get_mut() {
        for entry in memory_map_response.memmap_mut() {
            if entry.typ != limine::MemoryMapEntryType::Usable {
                continue;
            }
            free(entry.base as *mut Freelist);
        }
    } else {
        panic!("I can't get memory mapping information from the bootloader.\nIs it broken??");
    }
    if let Some(higher_half_response) = HHDM_REQUEST.get_response().get() {
        higher_half_response.offset;
    } else {
        panic!("I can't get HHDM information from the bootloader.\nIs it broken??");
    }
}

pub fn hhdm() -> VirtAddr {
    unsafe {
        if let Some(value) = HHDM {
            value
        } else {
            HHDM = if let Some(hhdm_response) = HHDM_REQUEST.get_response().get() {
                Some(VirtAddr::new(hhdm_response.offset))
            } else {
                panic!("I can't get HHDM information from the bootloader.\nI must have lost track of it.");
            };
            HHDM.unwrap()
        }
    }
}

unsafe impl FrameAllocator<Size4KiB> for Allocator {
    fn allocate_frame(&mut self) -> Option<PhysFrame<Size4KiB>> {
        allocate::<u64>().ok().map(|a| {
            PhysFrame::from_start_address(PhysAddr::new(a as u64))
                .expect("Allocated page that was not aligned")
        })
    }
}

pub unsafe fn current_page_map<'a>() -> OffsetPageTable<'a> {
    OffsetPageTable::new(
        &mut *(Cr3::read().0.start_address().as_u64() as *mut _),
        hhdm(),
    )
}