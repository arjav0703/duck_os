use x86_64::{
    PhysAddr, VirtAddr,
    registers::control::Cr3,
    structures::paging::{PageTable, PhysFrame, Size4KiB},
};

pub fn fetch_next_table(
    phy_mem_offset: VirtAddr,
    entry_table: PhysFrame<Size4KiB>,
) -> &'static mut PageTable {
    let phys = entry_table.start_address();
    let virt = phy_mem_offset + phys.as_u64();
    let page_table_ptr: *mut PageTable = virt.as_mut_ptr();

    unsafe { &mut *page_table_ptr }
}

pub fn fetch_l4_table(phy_mem_offset: VirtAddr) -> &'static mut PageTable {
    let (l4_table_frame, _) = Cr3::read();
    fetch_next_table(phy_mem_offset, l4_table_frame)
}
