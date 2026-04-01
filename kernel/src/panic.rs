use crate::exit::{QemuExitCode, exit_qemu};
use crate::println;
use crate::serial_println;
use core::panic::PanicInfo;

#[cfg(not(test))]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    println!("{}", info);
    serial_println!("Panic: {}\n", info);
    loop {}
}

#[cfg(test)]
#[panic_handler]
fn panic(info: &PanicInfo) -> ! {
    serial_println!("[ERROR]\n");
    serial_println!("{}\n", info);
    exit_qemu(QemuExitCode::Failed);
    loop {}
}
