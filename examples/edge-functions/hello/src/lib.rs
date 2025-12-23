use edge_hive_wasm::prelude::*;
use std::alloc::{alloc, dealloc, Layout};
use std::mem;

#[no_mangle]
pub extern "C" fn allocate(size: usize) -> *mut u8 {
    let layout = match Layout::from_size_align(size, mem::align_of::<u8>()) {
        Ok(l) => l,
        Err(_) => return std::ptr::null_mut(),
    };
    unsafe { alloc(layout) }
}

#[no_mangle]
pub extern "C" fn deallocate(ptr: *mut u8, size: usize) {
    if ptr.is_null() {
        return;
    }
    let layout = match Layout::from_size_align(size, mem::align_of::<u8>()) {
        Ok(l) => l,
        Err(_) => return,
    };
    unsafe { dealloc(ptr, layout) };
}

#[no_mangle]
pub extern "C" fn handle_request(req_ptr: *const u8, req_len: usize) -> *mut u8 {
    let req_bytes = unsafe { std::slice::from_raw_parts(req_ptr, req_len) };
    let req: Request = match serde_json::from_slice(req_bytes) {
        Ok(r) => r,
        Err(_) => {
            let resp = Response::text(400, "Bad Request");
            let resp_bytes = serde_json::to_vec(&resp).unwrap_or_default();
            let resp_ptr = allocate(resp_bytes.len());
            if resp_ptr.is_null() {
                return std::ptr::null_mut();
            }
            unsafe {
                std::ptr::copy_nonoverlapping(resp_bytes.as_ptr(), resp_ptr, resp_bytes.len());
            }
            let out_ptr = allocate(8);
            if out_ptr.is_null() {
                return std::ptr::null_mut();
            }
            unsafe {
                *(out_ptr as *mut u32) = resp_ptr as u32;
                *(out_ptr.add(4) as *mut u32) = resp_bytes.len() as u32;
            }
            return out_ptr;
        }
    };

    let resp = if req.path == "/hello" {
        Response::json(200, serde_json::json!({ "message": "Hello from WASM!" }))
    } else {
        Response::text(404, "Not Found")
    };

    let resp_bytes = serde_json::to_vec(&resp).unwrap_or_default();
    let resp_ptr = allocate(resp_bytes.len());
    if resp_ptr.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        std::ptr::copy_nonoverlapping(resp_bytes.as_ptr(), resp_ptr, resp_bytes.len());
    }

    // Return a pointer to a (ptr, len) tuple.
    let out_ptr = allocate(8);
    if out_ptr.is_null() {
        return std::ptr::null_mut();
    }
    unsafe {
        *(out_ptr as *mut u32) = resp_ptr as u32;
        *(out_ptr.add(4) as *mut u32) = resp_bytes.len() as u32;
    }
    out_ptr
}
