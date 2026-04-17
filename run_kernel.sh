cd kernel
cargo bootimage

cd ..

qemu-system-x86_64 -drive format=raw,file=target/x86_64-duck_os/debug/bootimage-kernel.bin -serial stdio
