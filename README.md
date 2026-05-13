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
- RAM based file system (work in progress)
- ELF executable loader (work in progress)


https://github.com/user-attachments/assets/7854a5e0-3fd9-4c16-b749-79e4fe84a153


## Testing (For reviewers)

To test the Duck OS kernel, you can use QEMU, a popular open-source emulator. Follow these steps:

1. Download the latest release from https://github.com/arjav0703/duck_os/releases
2. Install qemu (https://www.qemu.org/download/)
3. Run the following command in your terminal (make sure the filename is appropriate).

```bash
qemu-system-x86_64 -drive format=raw,file=bootimage-kernel.bin -serial stdio
```

### AI Declaration
Throughout the development of the project, I had tab completions turned on in my code editor (neovim btw :P). Occasionally used opencode to debug (some parts of the memory management section).
