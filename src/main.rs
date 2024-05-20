#![no_std] // Don't link the Rust standard library.
#![no_main] // Disable all Rust-level entry points.

use core::panic::PanicInfo;

mod vga_buffer;

static HELLO: &[u8] = b"Hello World!";

// Don't mangle the function.
#[no_mangle]
// This function is the entry point, since the link$
// er looks for a function named `_start` by default
pub extern "C" fn _start() -> ! {
    println!("Hello World{}", "!");
    panic!("Some panic message");

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
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    loop {}
}
