#![no_main]
#![no_std]

use core::slice;
use log::{info,error};
use uefi::{prelude::*, proto::{device_path::{text::{AllowShortcuts, DevicePathToText, DisplayOnly}, DevicePath}, loaded_image::LoadedImage}, table::boot::{self, MemoryMap, MemoryType}, Error, Identify};
use uefi::table::boot::SearchType::ByProtocol;

#[entry]
fn main(_image_handle: Handle, mut system_table:SystemTable<Boot>) -> Status {
    uefi::helpers::init(&mut system_table).unwrap();
    info!("Hello world!");

    match get_memory_map(&system_table) {
        Ok(map)=>{
            info!("Got memory map");
            info!("Start addr | Length in pages | Type");
            map.entries().for_each(|e| {
                info!("{:#08x}   | {:08x}       |", e.phys_start, e.page_count);
            });
            
        },
        Err(e)=>
            error!("Could not get memory map: {}", e)
    }

    print_image_path(system_table.boot_services());
    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}

fn get_memory_map(system_table:&SystemTable<Boot>) -> Result<MemoryMap, Error> {
    let map_buffer_size = system_table.boot_services().memory_map_size();
    info!("map_buffer_size {}", map_buffer_size.map_size);
    let map_buffer = system_table.boot_services()
        .allocate_pool( MemoryType::LOADER_DATA, map_buffer_size.map_size*2)?;

    let map_buffer_ref = unsafe {
        slice::from_raw_parts_mut(map_buffer, map_buffer_size.map_size*2)
    };

    let mut map = system_table.boot_services().memory_map(map_buffer_ref)?;
    map.sort();
    Ok(map)
}

fn print_image_path(boot_services:&BootServices) -> uefi::Result {
    let loaded_image = boot_services
        .open_protocol_exclusive::<LoadedImage>(boot_services.image_handle())?;

    let device_path_to_text_handle = *boot_services
        .locate_handle_buffer(ByProtocol(&DevicePathToText::GUID))?
        .first()
        .expect("DevicePathToText is missing");

    let device_path_to_text = boot_services
        .open_protocol_exclusive::<DevicePathToText>(device_path_to_text_handle)?;

    let image_device_path = loaded_image.file_path().expect("File path is not set");
    
    let image_device_path_text = device_path_to_text.convert_device_path_to_text(
        boot_services, image_device_path, DisplayOnly(true), AllowShortcuts(false))
        .expect("convert_path_to_text failed");

    info!("Image path: {}", &*image_device_path_text);
    Ok(())
}