use x86_64::instructions::interrupts;
use x86_64::instructions::segmentation::{Segment, CS, SS};
use x86_64::registers::segmentation::SegmentSelector;
use x86_64::structures::gdt::{Descriptor, GlobalDescriptorTable};
use x86_64::structures::tss::TaskStateSegment;

static mut GLOBAL_DESCRIPTOR_TABLE: GlobalDescriptorTable = GlobalDescriptorTable::new();
static mut TASK_STATE_SEGMENT: TaskStateSegment = TaskStateSegment::new();
pub unsafe fn build() {
    interrupts::disable();
    GLOBAL_DESCRIPTOR_TABLE.add_entry(Descriptor::kernel_code_segment());
    GLOBAL_DESCRIPTOR_TABLE.add_entry(Descriptor::kernel_data_segment());
    GLOBAL_DESCRIPTOR_TABLE.add_entry(Descriptor::user_code_segment());
    GLOBAL_DESCRIPTOR_TABLE.add_entry(Descriptor::user_data_segment());
    GLOBAL_DESCRIPTOR_TABLE.add_entry(Descriptor::tss_segment(&TASK_STATE_SEGMENT));
    GLOBAL_DESCRIPTOR_TABLE.load();
    CS::set_reg(SegmentSelector(0x08));
    SS::set_reg(SegmentSelector(0x10));
}
