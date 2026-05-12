use alloc::vec::Vec;

const ELF_MAGIC: [u8; 4] = [0x7f, b'E', b'L', b'F'];
const ELFCLASS64: u8 = 2;
const ELFDATA2LSB: u8 = 1;
const EM_X86_64: u16 = 62;
const PT_LOAD: u32 = 1;

#[derive(Debug)]
pub enum ExecError {
    InvalidFormat,
    Unsupported,
}

#[derive(Debug, Clone, Copy)]
pub struct SegmentInfo {
    pub vaddr: u64,
    pub filesz: u64,
    pub memsz: u64,
    pub flags: u32,
}

#[derive(Debug)]
pub struct ElfInfo {
    pub entry: u64,
    pub segments: Vec<SegmentInfo>,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Elf64Ehdr {
    e_ident: [u8; 16],
    e_type: u16,
    e_machine: u16,
    e_version: u32,
    e_entry: u64,
    e_phoff: u64,
    e_shoff: u64,
    e_flags: u32,
    e_ehsize: u16,
    e_phentsize: u16,
    e_phnum: u16,
    e_shentsize: u16,
    e_shnum: u16,
    e_shstrndx: u16,
}

#[repr(C)]
#[derive(Clone, Copy)]
struct Elf64Phdr {
    p_type: u32,
    p_flags: u32,
    p_offset: u64,
    p_vaddr: u64,
    p_paddr: u64,
    p_filesz: u64,
    p_memsz: u64,
    p_align: u64,
}

pub fn parse_elf(image: &[u8]) -> Result<ElfInfo, ExecError> {
    if image.len() < core::mem::size_of::<Elf64Ehdr>() {
        return Err(ExecError::InvalidFormat);
    }

    let ehdr: Elf64Ehdr = read_struct(image, 0)?;
    if ehdr.e_ident[0..4] != ELF_MAGIC {
        return Err(ExecError::InvalidFormat);
    }
    if ehdr.e_ident[4] != ELFCLASS64 || ehdr.e_ident[5] != ELFDATA2LSB {
        return Err(ExecError::Unsupported);
    }
    if ehdr.e_machine != EM_X86_64 {
        return Err(ExecError::Unsupported);
    }

    let phoff = ehdr.e_phoff as usize;
    let phentsize = ehdr.e_phentsize as usize;
    let phnum = ehdr.e_phnum as usize;
    let ph_end = phoff.checked_add(phentsize * phnum).ok_or(ExecError::InvalidFormat)?;
    if ph_end > image.len() {
        return Err(ExecError::InvalidFormat);
    }

    let mut segments: Vec<SegmentInfo> = Vec::new();
    for i in 0..phnum {
        let base = phoff + i * phentsize;
        let phdr: Elf64Phdr = read_struct(image, base)?;
        if phdr.p_type != PT_LOAD {
            continue;
        }

        let file_end = phdr
            .p_offset
            .checked_add(phdr.p_filesz)
            .ok_or(ExecError::InvalidFormat)? as usize;
        if file_end > image.len() {
            return Err(ExecError::InvalidFormat);
        }
        let _data = &image[phdr.p_offset as usize..file_end];
        segments.push(SegmentInfo {
            vaddr: phdr.p_vaddr,
            filesz: phdr.p_filesz,
            memsz: phdr.p_memsz,
            flags: phdr.p_flags,
        });
    }

    Ok(ElfInfo {
        entry: ehdr.e_entry,
        segments,
    })
}

fn read_struct<T: Copy>(data: &[u8], offset: usize) -> Result<T, ExecError> {
    let size = core::mem::size_of::<T>();
    let end = offset.checked_add(size).ok_or(ExecError::InvalidFormat)?;
    if end > data.len() {
        return Err(ExecError::InvalidFormat);
    }
    let ptr = unsafe { data.as_ptr().add(offset) as *const T };
    Ok(unsafe { core::ptr::read_unaligned(ptr) })
}
