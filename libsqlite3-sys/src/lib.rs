#![allow(non_snake_case, non_camel_case_types)]
#![cfg_attr(test, allow(deref_nullptr))] // https://github.com/rust-lang/rust-bindgen/issues/2066

// force linking to openssl
#[cfg(feature = "bundled-sqlcipher-vendored-openssl")]
extern crate openssl_sys;

pub use self::error::*;

use std::default::Default;
use std::mem;

mod error;

#[must_use]
pub fn SQLITE_STATIC() -> sqlite3_destructor_type {
    None
}

#[must_use]
pub fn SQLITE_TRANSIENT() -> sqlite3_destructor_type {
    Some(unsafe { mem::transmute(-1_isize) })
}

#[allow(clippy::all)]
mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindgen.rs"));
}
pub use bindings::*;

pub type sqlite3_index_constraint = sqlite3_index_info_sqlite3_index_constraint;
pub type sqlite3_index_constraint_usage = sqlite3_index_info_sqlite3_index_constraint_usage;

impl Default for sqlite3_vtab {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

impl Default for sqlite3_vtab_cursor {
    fn default() -> Self {
        unsafe { mem::zeroed() }
    }
}

#[cfg(all(target_arch = "wasm32", target_os = "unknown"))]
mod allocator {

    #[no_mangle]
    pub unsafe extern "C" fn malloc(len: usize) -> *mut u8 {
        let align = std::mem::align_of::<usize>();
        let layout = std::alloc::Layout::from_size_align_unchecked(len, align);

        std::alloc::alloc(layout)
    }

    const SQLITE_PTR_SIZE: usize = 8;

    #[no_mangle]
    pub unsafe extern "C" fn free(ptr: *mut u8) {
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
}
