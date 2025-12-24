//! Hello World Edge Function
//!
//! A simple example edge function that demonstrates the basic structure
//! required for Edge Hive WASM functions.

use serde::{Deserialize, Serialize};
use serde_json::Value;

#[derive(Debug, Serialize, Deserialize)]
struct Request {
    name: Option<String>,
}

#[derive(Debug, Serialize, Deserialize)]
struct Response {
    message: String,
    timestamp: u64,
}

// Global allocator using wee_alloc for small binary size
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

/// Allocate memory for WASM
///
/// This is called by the host to allocate memory for passing data to/from WASM.
#[no_mangle]
pub extern "C" fn allocate(size: i32) -> i32 {
    let vec = Vec::<u8>::with_capacity(size as usize);
    let ptr = vec.as_ptr() as i32;
    std::mem::forget(vec);
    ptr
}

/// Deallocate memory
#[no_mangle]
pub extern "C" fn deallocate(ptr: i32, size: i32) {
    unsafe {
        let _ = Vec::<u8>::from_raw_parts(ptr as *mut u8, size as usize, size as usize);
    }
}

/// Main entry point for the edge function
///
/// # Arguments
/// * `req_ptr` - Pointer to request JSON string
/// * `req_len` - Length of request JSON string
///
/// # Returns
/// i64 with response pointer in lower 32 bits and length in upper 32 bits
#[no_mangle]
pub extern "C" fn handle_request(req_ptr: i32, req_len: i32) -> i64 {
    // Read request from memory
    let request_json = unsafe {
        let slice = std::slice::from_raw_parts(req_ptr as *const u8, req_len as usize);
        std::str::from_utf8(slice).unwrap_or("{}")
    };

    // Parse request
    let request: Value = serde_json::from_str(request_json).unwrap_or(serde_json::json!({}));
    let req_obj: Request = serde_json::from_value(request).unwrap_or(Request { name: None });

    // Create response
    let name = req_obj.name.unwrap_or_else(|| "World".to_string());
    let response = Response {
        message: format!("Hello, {}!", name),
        timestamp: 0, // In a real implementation, this would come from the host
    };

    // Serialize response
    let response_json = serde_json::to_string(&response).unwrap();
    let response_bytes = response_json.into_bytes();
    let response_len = response_bytes.len() as i32;

    // Allocate memory for response
    let resp_ptr = allocate(response_len);

    // Write response to memory
    unsafe {
        let dest = std::slice::from_raw_parts_mut(resp_ptr as *mut u8, response_len as usize);
        dest.copy_from_slice(&response_bytes);
    }

    // Pack ptr and len into i64
    ((response_len as i64) << 32) | (resp_ptr as i64)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_deallocate() {
        let ptr1 = allocate(100);
        assert!(ptr1 >= 0);
        
        deallocate(ptr1, 100);
    }
}
