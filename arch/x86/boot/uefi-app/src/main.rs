#![no_main]
#![no_std]

use core::panic::PanicInfo;
use uefi::{
    prelude::*,
    println,
    proto::media::{
        file::{Directory, File, FileAttribute, FileMode},
        fs::SimpleFileSystem,
    },
    table::boot::{AllocateType, MemoryMap, MemoryType, SearchType, PAGE_SIZE},
    Identify, Result,
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

    let memory_map = unsafe { get_memory_map(system_table.boot_services()) };
    let mut fs_volume = get_fs_volume(system_table.boot_services()).unwrap();

    let mut file = fs_volume
        .open(
            cstr16!("EFI"),
            FileMode::Read,
            FileAttribute::READ_ONLY | FileAttribute::HIDDEN | FileAttribute::SYSTEM,
        )
        .unwrap()
        .into_directory()
        .unwrap()
        .open(
            cstr16!("BOOT"),
            FileMode::Read,
            FileAttribute::READ_ONLY | FileAttribute::HIDDEN | FileAttribute::SYSTEM,
        )
        .unwrap()
        .into_directory()
        .unwrap()
        .open(
            cstr16!("BOOTX64.EFI"),
            FileMode::Read,
            FileAttribute::READ_ONLY | FileAttribute::HIDDEN | FileAttribute::SYSTEM,
        )
        .unwrap()
        .into_regular_file()
        .unwrap();
    let mut buf = [0u8; 4];
    file.read(&mut buf).unwrap();
    println!("{:02X}", buf[0]);

    let mut i = 0;
    for entry in memory_map.entries() {
        println!(
            "{:X} - {:X} - {:?}",
            entry.phys_start,
            entry.phys_start + (entry.page_count * PAGE_SIZE as u64),
            entry.ty
        );
        i += 1;
        if i > 16 {
            break;
        }
    }

    system_table.boot_services().stall(usize::MAX);

    Status::SUCCESS
}

unsafe fn get_memory_map<'buf>(boot_services: &BootServices) -> MemoryMap<'buf> {
    let memory_map_size = boot_services.memory_map_size();
    let memory_map_pages = (memory_map_size.map_size / PAGE_SIZE) + 1;
    let memory_map_buffer = boot_services
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

    boot_services.memory_map(memory_map_buffer).unwrap()
}

fn get_fs_volume(boot_services: &BootServices) -> Result<Directory> {
    let simple_fs_handle = *boot_services
        .locate_handle_buffer(SearchType::ByProtocol(&SimpleFileSystem::GUID))?
        .first()
        .expect("SimpleFileSystem protocol is missing");

    let mut simple_fs =
        boot_services.open_protocol_exclusive::<SimpleFileSystem>(simple_fs_handle)?;

    simple_fs.open_volume()
}
