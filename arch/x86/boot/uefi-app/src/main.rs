#![no_main]
#![no_std]

use core::panic::PanicInfo;
use uefi::{
    prelude::*,
    println,
    table::boot::{AllocateType, MemoryType, PAGE_SIZE},
};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();

    system_table.stdout().clear().unwrap();

    let memory_map_size = system_table.boot_services().memory_map_size();
    let memory_map_pages = (memory_map_size.map_size / PAGE_SIZE) + 1;
    let memory_map_buffer = system_table
        .boot_services()
        .allocate_pages(
            AllocateType::AnyPages,
            MemoryType::RUNTIME_SERVICES_DATA,
            memory_map_pages,
        )
        .expect("Could not allocate pages for memory map");
    let memory_map_buffer = unsafe {
        let ptr = memory_map_buffer as *mut u8;
        core::slice::from_raw_parts_mut(ptr, memory_map_pages * PAGE_SIZE)
    };
    let memory_map = system_table
        .boot_services()
        .memory_map(memory_map_buffer)
        .unwrap();
    let mut i = 0;
    for entry in memory_map.entries() {
        println!(
            "{:X} - {:X} - {:?}",
            entry.phys_start,
            entry.phys_start + (entry.page_count * PAGE_SIZE as u64),
            entry.ty
        );
        i += 1;
        if i > 24 {
            break;
        }
    }

    system_table.boot_services().stall(usize::MAX);

    Status::SUCCESS
}
