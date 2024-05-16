#![no_std] // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.

use core::panic::PanicInfo;

static HELLO: &[u8] = b"Hello World!";

// Don't mangle the function.
#[no_mangle]
// This function is the entry point, since the link$
// er looks for a function named `_start` by default
pub extern "C" fn _start() -> ! {
    // Cast the integer into a raw pointer.
    let vga_buffer = 0xb8000 as *mut u8;

    for (i, &byte) in HELLO.iter().enumerate() {
        // Use offset() to write the string byte and color byte.
        //
        // Tell the compiler that the operations are valid.
        unsafe {
            *vga_buffer.offset(i as isize * 2) = byte;
            *vga_buffer.offset(i as isize * 2 + 1) = 0xb;
        }
    }

    loop {}
}

/// This function is called on panic.
// The PanicInfo parameter contains the file and lin
// e where the panic happened and the optional panic
// message.
//
// The function should never return, so it is marked
// as a diverging function by returning the “neve"
// type.
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
