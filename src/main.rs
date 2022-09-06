#![no_std] //disable std library
#![no_main] // don't use main() function as entry point

// import necessary items
use core::panic::PanicInfo;

// function to handle panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
// this function is entry point
#[no_mangle] // disable function name mangling
pub extern "C" fn _start() -> ! {
    loop {}
}
