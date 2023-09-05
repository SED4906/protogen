#![feature(abi_x86_interrupt)]
#![feature(error_in_core)]
#![feature(panic_info_message)]
#![no_std]
#![no_main]

use x86_64::instructions;
use x86_64::instructions::interrupts;

mod process;
mod framebuffer;
mod memory;
mod terminal;
mod gdt;
mod idt;

static IMAGE: &[u8;8720] = include_bytes!("testing_elf");

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
    process::spawn(IMAGE);
    hcf();
}

#[panic_handler]
fn rust_panic(_info: &core::panic::PanicInfo) -> ! {
    if let Some(message) = _info.message() {
        println!();
        println!("                             \n");
        println!("                             \n");
        println!("    FLAGRANT SYSTEM ERROR    \n");
        println!("       Computer over.        \n");
        println!("      Panic = Very Yes.      \n");
        println!("                             \n");
        println!("                             \n");
        println!("                             \n");
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
