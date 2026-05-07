# Duck OS

An Operating System Kernel written in Rust (x86_64 architecture).

<img width="360" height="222" alt="image" src="https://github.com/user-attachments/assets/21efc5f7-54b3-4132-92ca-18e9b631d5dc" />

## Features
- Memory management (paging, heap allocation)
- Legacy UEFI boot support
- Interrupt handling
- ANSI keyboard input
- VGA text mode output
- Basic Shell (work in progress)


## Testing (For reviewers)

To test the Duck OS kernel, you can use QEMU, a popular open-source emulator. Follow these steps:

1. Download the latest release from https://github.com/arjav0703/duck_os/releases
2. Install qemu (https://www.qemu.org/download/)
3. Run the following command in your terminal (make sure the filename is appropriate).

```bash
qemu-system-x86_64 -drive format=raw,file=bootimage-kernel.bin -serial stdio
```
