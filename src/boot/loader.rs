#![no_std]
#![no_main]

use uefi::prelude::*;
use core::panic::PanicInfo;
use kernel::kernel::kernel_entry;

#[entry]
fn main(_image_handle: Handle, mut system_table: SystemTable<Boot>) -> Status {
    if let Err(_) = uefi::helpers::init(&mut system_table) {
        return Status::UNSUPPORTED;
    }
    
    let _ = system_table.stdout().clear();
    let _ = system_table
        .stdout()
        .output_string(cstr16!("kyou ha oyakodon!\n"));
    
    //ブートサービスの終了
    let _ = unsafe {
        system_table.exit_boot_services(uefi::table::boot::MemoryType::LOADER_DATA)
    };
    
    kernel_entry();
    
    Status::SUCCESS
}

#[global_allocator]
static ALLOCATOR: uefi::allocator::Allocator = uefi::allocator::Allocator;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}