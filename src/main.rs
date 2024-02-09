#![no_std] // Don't link rust std as it is os dependent
#![no_main] // Disable rust entry points because they depend on c libs

use core::panic::PanicInfo;

#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}

static HELLO: &[u8] = b"Hello Madam President Madelyn Khoury!";

// Custom entrypoint that doesn't require c libraries
#[no_mangle]
pub extern "C" fn _start() -> ! {
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xd;
        }
    }

    loop {}
}
