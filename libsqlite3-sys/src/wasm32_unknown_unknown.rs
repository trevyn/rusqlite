#[cfg(not(feature = "bundled"))]
compile_error!("wasm32-unknown-unknown must be built with '--features bundled'");

#[no_mangle]
pub unsafe extern "C" fn sqlite3_os_init() -> std::os::raw::c_int {
    let vfs = crate::sqlite3_vfs {
        iVersion: 1,
        szOsFile: 0,
        mxPathname: 1024,
        pNext: std::ptr::null_mut(),
        zName: "libsqlite3-sys".as_ptr() as *const std::os::raw::c_char,
        pAppData: std::ptr::null_mut(),
        xOpen: None,                       //Some(vfs::dss_open),
        xDelete: None,                     //Some(vfs::dss_delete),
        xAccess: None,                     //Some(vfs::dss_access),
        xFullPathname: None,               //Some(vfs::dss_full_path_name),
        xDlOpen: None,                     //Some(vfs::dss_dl_open),
        xDlError: None,                    //Some(vfs::dss_dl_error),
        xDlSym: None,                      //Some(vfs::dss_dl_sym),
        xDlClose: None,                    //Some(vfs::dss_dl_close),
        xRandomness: Some(vfs_randomness), //Some(vfs::dss_randomness),
        xSleep: None,                      //Some(vfs::dss_sleep),
        xCurrentTime: None,                //Some(vfs::dss_current_time),
        xGetLastError: None,               //Some(vfs::dss_get_last_error),
        xCurrentTimeInt64: None,
        xSetSystemCall: None,
        xGetSystemCall: None,
        xNextSystemCall: None,
    };

    crate::sqlite3_vfs_register(Box::leak(Box::new(vfs)), 1)
}

const fn max(a: usize, b: usize) -> usize {
    [a, b][(a < b) as usize]
}

const ALIGN: usize = max(8, std::mem::align_of::<usize>());

#[no_mangle]
pub unsafe extern "C" fn malloc(size: usize) -> *mut u8 {
    let layout = match std::alloc::Layout::from_size_align(size + ALIGN, ALIGN) {
        Ok(layout) => layout,
        Err(_) => return std::ptr::null_mut(),
    };

    let ptr = std::alloc::alloc(layout);
    if ptr.is_null() {
        return std::ptr::null_mut();
    }

    *(ptr as *mut usize) = size;
    ptr.offset(ALIGN as isize)
}

#[no_mangle]
pub unsafe extern "C" fn free(ptr: *mut u8) {
    let ptr = ptr.offset(-(ALIGN as isize));
    let size = *(ptr as *mut usize);
    let layout = std::alloc::Layout::from_size_align_unchecked(size + ALIGN, ALIGN);

    std::alloc::dealloc(ptr, layout);
}

#[no_mangle]
pub unsafe extern "C" fn realloc(ptr: *mut u8, new_size: usize) -> *mut u8 {
    let ptr = ptr.offset(-(ALIGN as isize));
    let size = *(ptr as *mut usize);
    let layout = std::alloc::Layout::from_size_align_unchecked(size + ALIGN, ALIGN);

    let ptr = std::alloc::realloc(ptr, layout, new_size + ALIGN);
    if ptr.is_null() {
        return std::ptr::null_mut();
    }

    *(ptr as *mut usize) = new_size;
    ptr.offset(ALIGN as isize)
}

#[no_mangle]
pub unsafe extern "C" fn vfs_randomness(
    _arg1: *mut crate::sqlite3_vfs,
    n_byte: std::os::raw::c_int,
    z_out: *mut std::os::raw::c_char,
) -> std::os::raw::c_int {
    let slice = std::slice::from_raw_parts_mut(z_out as *mut u8, n_byte as usize);
    getrandom::getrandom(slice).unwrap();
    crate::SQLITE_OK
}
