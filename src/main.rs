#![no_std]
#![no_main]

use core::panic::PanicInfo;

// Custom entrypoint that doesn't link to c libraries
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
