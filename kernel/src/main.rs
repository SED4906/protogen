#![feature(abi_x86_interrupt)]
#![feature(error_in_core)]
#![feature(panic_info_message)]
#![feature(concat_bytes)]
#![feature(slice_as_chunks)]
#![feature(ptr_from_ref)]
#![no_std]
#![no_main]

use x86_64::instructions;
use x86_64::instructions::interrupts;

mod framebuffer;
mod gdt;
mod idt;
mod memory;
mod process;
mod terminal;

#[repr(C, align(4096))]
pub struct A4096;
static TEST_IMAGE: (A4096, [u8; 4096]) = (A4096, *concat_bytes!([0xEB, 0xFE], [0; 4094]));

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    framebuffer::clear(0xFF111111);
    println!("greetings");
    memory::build();
    println!("memory map built");
    gdt::build();
    println!("global descriptor table built");
    idt::build();
    interrupts::enable();
    println!("interrupt descriptor table built");
    let proc = process::create_process(&TEST_IMAGE.1).expect("couldn't create process");
    process::PROCESSES = Some(proc);
    process::CURRENT_PROCESS = Some(process::PROCESSES.as_mut().unwrap());
    process::enter_task();
    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    if let Some(message) = _info.message() {
        println!();
        println!("                             ");
        println!("                             ");
        println!("    FLAGRANT SYSTEM ERROR    ");
        println!("       Computer over.        ");
        println!("      Panic = Very Yes.      ");
        println!("                             ");
        println!("                             ");
        println!("                             ");
        println!("{message}");
    }
    hcf();
}

fn hcf() -> ! {
    interrupts::disable();
    loop {
        instructions::hlt();
    }
}
