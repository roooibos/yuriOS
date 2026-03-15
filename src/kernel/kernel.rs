use core::panic::PanicInfo;
use crate::*;
use crate::monitor::draw_sikaku;

#[unsafe(no_mangle)]
pub extern "C" fn kernel_entry(boot_info: &'static BootInfo) -> ! {
    draw_sikaku(&boot_info, 100, 100, 700, 250, 0x0067A7CC);
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}