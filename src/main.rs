#![no_main]
#![no_std]

use core::slice;
use log::{info,error};
use uefi::{prelude::*, table::boot::{MemoryMap, MemoryType}, Error};


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

    system_table.boot_services().stall(10_000_000);
    Status::SUCCESS
}

fn get_memory_map(system_table:&SystemTable<Boot>) -> Result<MemoryMap, Error> {
    let map_buffer_size = system_table.boot_services().memory_map_size();
    info!("map_buffer_size {}", map_buffer_size.map_size);
    let map_buffer = system_table.boot_services()
        .allocate_pool( MemoryType::LOADER_DATA, map_buffer_size.map_size*2)?;

    // let map_buffer_ref = unsafe {
    //     map_buffer.as_mut().unwrap()
    // };

    let map_buffer_ref = unsafe {
        slice::from_raw_parts_mut(map_buffer, map_buffer_size.map_size*2)
    };

    let mut map = system_table.boot_services().memory_map(map_buffer_ref)?;
    map.sort();
    Ok(map)
}