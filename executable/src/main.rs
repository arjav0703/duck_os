#![no_std]
#![no_main]

use kernel::println;

#[unsafe(no_mangle)]
pub extern "C" fn _start() -> ! {
    println!("Hello from executable");
    loop {
        x86_64::instructions::hlt();
    }
}

// use core::panic::PanicInfo;
//
// #[panic_handler]
// fn panic(_info: &PanicInfo) -> ! {
//     loop {
//         x86_64::instructions::hlt();
//     }
// }
