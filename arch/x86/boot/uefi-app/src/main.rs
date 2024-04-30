#![no_main]
#![no_std]

use core::panic::PanicInfo;
use uefi::{prelude::*, println};

#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("PANIC!");
    loop {}
}

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();

    println!("Test");

    system_table.boot_services().stall(5 * 1_000_000);

    Status::SUCCESS
}
