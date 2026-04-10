#[derive(Debug, Clone, Copy, PartialEq, Eq)]
#[repr(u32)]
pub enum QemuExitCode {
    Success = 0x10,
    Failed = 0x11,
}

pub fn exit_qemu(exit_code: QemuExitCode) {
    use x86_64::instructions::port::Port;

    unsafe {
        let mut port = Port::new(0xf4);
        port.write(exit_code as u32);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test_case]
    fn test_qemu_exit_code_success_value() {
        // 0x10 << 1 | 1 = 33, which matches test-success-exit-code in Cargo.toml
        assert_eq!(QemuExitCode::Success as u32, 0x10);
    }

    #[test_case]
    fn test_qemu_exit_code_failed_value() {
        assert_eq!(QemuExitCode::Failed as u32, 0x11);
    }

    #[test_case]
    fn test_qemu_exit_codes_are_not_equal() {
        assert_ne!(QemuExitCode::Success, QemuExitCode::Failed);
    }
}
