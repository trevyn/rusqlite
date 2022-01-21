#[no_mangle]
pub unsafe extern "C" fn malloc(len: usize) -> *mut u8 {
    let align = std::mem::align_of::<usize>();
    let layout = std::alloc::Layout::from_size_align_unchecked(len, align);

    std::alloc::alloc(layout)
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    const SQLITE_PTR_SIZE: usize = 8;
    // The SQLite allocator stores the length in the first 8 bytes of the allocation.
    // We re-use that to satisfy Rust's desire to know the Layout in dealloc().
    // See https://sqlite.org/malloc.html#the_default_memory_allocator

    let mut size_a = [0; SQLITE_PTR_SIZE];

    size_a.as_mut_ptr().copy_from(ptr, SQLITE_PTR_SIZE);

    let ptr_size: u64 = u64::from_le_bytes(size_a);

    let align = std::mem::align_of::<usize>();
    let layout = std::alloc::Layout::from_size_align_unchecked(ptr_size as usize, align);

    std::alloc::dealloc(ptr, layout);
}

#[no_mangle]
pub unsafe extern "C" fn realloc(ptr: *mut u8, size: usize) -> *mut u8 {
    let align = std::mem::align_of::<usize>();
    let layout = std::alloc::Layout::from_size_align_unchecked(size, align);

    std::alloc::realloc(ptr, layout, size)
}
