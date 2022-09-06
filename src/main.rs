//disable std library
#![no_std]

// import necessary items
use core::panic::PanicInfo;

// function to handle panic
#[panic_handler]
fn panic(_info: &PanicInfo) -> !  {
    loop {}
}

fn main() {}

