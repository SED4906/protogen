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
mod pic;
mod process;
mod terminal;

#[repr(C, align(4096))]
pub struct A4096;
static TEST_IMAGE: (A4096, [u8; 2]) = (A4096, [0xEB, 0xFE]);

#[no_mangle]
unsafe extern "C" fn _start() -> ! {
    framebuffer::clear(0xFF111111);
    println!("greetings");
    memory::build();
    println!("memory map built");
    gdt::build();
    println!("global descriptor table built");
    idt::build();
    println!("interrupt descriptor table built");
    process::store_kernel_pagemap();
    let test_image = memory::allocate::<[u8; 4096]>().expect("couldn't allocate memory");
    test_image.as_mut().unwrap()[0..2].copy_from_slice(&TEST_IMAGE.1);
    process::create_process(&*test_image).expect("couldn't create process");
    let test_image2 = memory::allocate::<[u8; 4096]>().expect("couldn't allocate memory");
    test_image2.as_mut().unwrap()[0..2].copy_from_slice(&TEST_IMAGE.1);
    process::create_process(&*test_image2).expect("couldn't create process");
    let test_image3 = memory::allocate::<[u8; 4096]>().expect("couldn't allocate memory");
    test_image3.as_mut().unwrap()[0..2].copy_from_slice(&TEST_IMAGE.1);
    process::create_process(&*test_image3).expect("couldn't create process");
    interrupts::enable();
    println!("enabled interrupts");
    loop {
        process::enter_task();
        interrupts::disable();
        if let Some(current_process) = process::CURRENT_PROCESS {
            process::CURRENT_PROCESS = Some((*current_process).next.unwrap())
        } else {
            panic!("No processes");
        }
        interrupts::enable();
    }
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
