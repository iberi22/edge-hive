use std::alloc::{Layout, alloc, dealloc};

extern "C" {
    fn db_query(sql_ptr: *const u8, sql_len: i32) -> i32;
    fn log(level: i32, msg_ptr: *const u8, msg_len: i32);
}

#[no_mangle]
pub extern "C" fn allocate(size: usize) -> *mut u8 {
    let layout = Layout::from_size_align(size, 1).unwrap();
    unsafe { alloc(layout) }
}

#[no_mangle]
pub extern "C" fn deallocate(ptr: *mut u8, size: usize) {
    let layout = Layout::from_size_align(size, 1).unwrap();
    unsafe { dealloc(ptr, layout) };
}

#[no_mangle]
pub extern "C" fn validate_login(email_ptr: *const u8, email_len: i32) -> i32 {
    let email = unsafe {
        core::slice::from_raw_parts(email_ptr, email_len as usize)
    };

    if email.ends_with(b"@mycompany.com") {
        1
    } else {
        0
    }
}
