//disable std library
#![no_std]
// don't use main() function as entry point
#![no_main]

// import necessary items
use core::panic::PanicInfo;

// function to handle panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> ! {
    loop {}
}
// entry point for our program
// disable function name mangling               
#[no_mangle]
pub extern "C" fn _start() -> ! {
    loop {}
}
