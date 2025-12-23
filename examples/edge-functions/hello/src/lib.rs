//! Hello World Edge Function
//!
//! A simple example edge function that demonstrates the basic structure
//! and memory management required for Edge Hive WASM functions.

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

// Simple allocator for strings
static mut HEAP: [u8; 64 * 1024] = [0; 64 * 1024];
static mut HEAP_POS: usize = 0;

/// Allocate memory for WASM
///
/// This is called by the host to allocate memory for passing data to/from WASM.
#[no_mangle]
pub extern "C" fn allocate(size: i32) -> i32 {
    unsafe {
        let pos = HEAP_POS as i32;
        HEAP_POS += size as usize;
        
        // Store size at the beginning (for deallocate)
        let size_bytes = size.to_le_bytes();
        HEAP[pos as usize] = size_bytes[0];
        HEAP[pos as usize + 1] = size_bytes[1];
        HEAP[pos as usize + 2] = size_bytes[2];
        HEAP[pos as usize + 3] = size_bytes[3];
        
        pos + 4 // Return pointer after size header
    }
}

/// Deallocate memory (simple implementation - just tracks top of heap)
#[no_mangle]
pub extern "C" fn deallocate(ptr: i32, _size: i32) {
    // In a real implementation, you'd want a proper allocator
    // For this simple example, we just reset if deallocating the last allocation
    unsafe {
        if ptr as usize <= HEAP_POS {
            HEAP_POS = (ptr - 4) as usize;
        }
    }
}

/// Main entry point for the edge function
///
/// # Arguments
/// * `req_ptr` - Pointer to request JSON string
/// * `req_len` - Length of request JSON string
///
/// # Returns
/// Pointer to response JSON string (with length stored at ptr-4)
#[no_mangle]
pub extern "C" fn handle_request(req_ptr: i32, req_len: i32) -> i32 {
    // Read request from memory
    let request_json = unsafe {
        let slice = &HEAP[req_ptr as usize..(req_ptr as usize + req_len as usize)];
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
    let response_len = response_json.len() as i32;

    // Allocate memory for response (including size header)
    let resp_ptr = allocate(response_len);

    // Write response to memory
    unsafe {
        let response_bytes = response_json.as_bytes();
        HEAP[resp_ptr as usize..resp_ptr as usize + response_len as usize]
            .copy_from_slice(response_bytes);
    }

    resp_ptr
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocate_deallocate() {
        let ptr1 = allocate(100);
        assert!(ptr1 >= 4); // Should be after size header
        
        deallocate(ptr1, 100);
    }
}
