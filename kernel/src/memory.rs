use x86_64::VirtAddr;

struct Freelist(*mut Freelist);
static mut FREELIST: Freelist = Freelist(core::ptr::null_mut());
static MEMMAP_REQUEST: limine::MemmapRequest = limine::MemmapRequest::new(0);
static HHDM_REQUEST: limine::HhdmRequest = limine::HhdmRequest::new(0);

static mut HHDM: Option<u64> = None;

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

pub fn allocate<T>() -> Result<*mut T, MemoryError> {
    unsafe {
        if FREELIST.0.is_null() {
            return Err(MemoryError::OutOfMemory);
        }
        let next = &mut *FREELIST.0;
        let current = FREELIST.0;
        FREELIST.0 = next.0;
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
    if let Some(memmap_response) = MEMMAP_REQUEST.get_response().get_mut() {
        for entry in memmap_response.memmap_mut() {
            if entry.typ != limine::MemoryMapEntryType::Usable {
                continue;
            }
            let mut address = entry.base;
            while address < entry.base + entry.len {
                free(address as *mut Freelist);
                address += 4096;
            }
        }
    } else {
        panic!("I can't get memory mapping information from the bootloader.\nIs it broken??");
    }
    if let Some(hhdm_response) = HHDM_REQUEST.get_response().get() {
        unsafe {
            HHDM = Some(hhdm_response.offset);
        }
    } else {
        panic!("I can't get HHDM information from the bootloader.\nIs it broken??");
    }
}

#[allow(dead_code)]
pub fn hhdm() -> VirtAddr {
    unsafe {
        if let Some(value) = HHDM {
            VirtAddr::from_ptr(value as *const u64)
        } else {
            panic!("I can't get the saved HHDM information somehow.\nI must have lost track of it.\nThis should never happen");
        }
    }
}

unsafe fn map_step(pagemap: &mut [u64; 512], entry: usize) -> Option<&mut [u64; 512]> {
    if pagemap[entry] & 1 == 0u64 {
        pagemap[entry] = allocate::<[u64; 512]>().ok()? as u64 | 7;
        (*((pagemap[entry] & !0xFFF) as *mut [u64; 512])).fill(0);
    }
    Some(&mut *((pagemap[entry] & !0xFFF) as *mut [u64; 512]))
}

pub unsafe fn map_to(pagemap: &mut [u64; 512], vaddr: u64, paddr: u64, flags: u64) -> Option<u64> {
    let entry_l4 = ((vaddr >> 39) & 0x1FF) as usize;
    let entry_l3 = ((vaddr >> 30) & 0x1FF) as usize;
    let entry_l2 = ((vaddr >> 21) & 0x1FF) as usize;
    let entry_l1 = ((vaddr >> 12) & 0x1FF) as usize;

    let pml3 = map_step(pagemap, entry_l4)?;
    let pml2 = map_step(pml3, entry_l3)?;
    let pml1 = map_step(pml2, entry_l2)?;

    pml1[entry_l1] = paddr | flags;
    Some(paddr)
}

#[allow(dead_code)]
unsafe fn translate_step(pagemap: &mut [u64; 512], entry: usize) -> Option<&mut [u64; 512]> {
    if pagemap[entry] & 1 == 0u64 {
        None
    } else {
        Some(&mut *((pagemap[entry] & !0xFFF) as *mut [u64; 512]))
    }
}

#[allow(dead_code)]
pub unsafe fn translate_page(pagemap: &mut [u64; 512], vaddr: u64) -> Option<u64> {
    let entry_l4 = ((vaddr >> 39) & 0x1FF) as usize;
    let entry_l3 = ((vaddr >> 30) & 0x1FF) as usize;
    let entry_l2 = ((vaddr >> 21) & 0x1FF) as usize;
    let entry_l1 = ((vaddr >> 12) & 0x1FF) as usize;

    let pml3 = translate_step(pagemap, entry_l4)?;
    let pml2 = translate_step(pml3, entry_l3)?;
    let pml1 = translate_step(pml2, entry_l2)?;

    if pml1[entry_l1] & 1 == 0u64 {
        None
    } else {
        Some(pml1[entry_l1] & !0xFFF)
    }
}
