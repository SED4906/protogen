use x86_64::structures::gdt::{GlobalDescriptorTable,Descriptor};
use x86_64::instructions::segmentation::{Segment,CS,SS};
use x86_64::registers::segmentation::SegmentSelector;
use x86_64::instructions::interrupts;

static mut GLOBAL_DESCRIPTOR_TABLE: GlobalDescriptorTable = GlobalDescriptorTable::new();
pub fn build() {
    unsafe {
        interrupts::disable();
        GLOBAL_DESCRIPTOR_TABLE.add_entry(Descriptor::kernel_code_segment());
        GLOBAL_DESCRIPTOR_TABLE.add_entry(Descriptor::kernel_data_segment());
        GLOBAL_DESCRIPTOR_TABLE.add_entry(Descriptor::user_code_segment());
        GLOBAL_DESCRIPTOR_TABLE.add_entry(Descriptor::user_data_segment());
        GLOBAL_DESCRIPTOR_TABLE.load();
        CS::set_reg(SegmentSelector(0x08));
        SS::set_reg(SegmentSelector(0x10));
    }
}