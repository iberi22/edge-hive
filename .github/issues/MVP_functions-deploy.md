---
title: "[MVP] Functions WASM Deploy & Management"
labels:
  - enhancement
  - backend
  - jules
  - P2
assignees: []
---

## Description

Enable uploading and managing WASM edge functions.

## Tasks

- [ ] Implement `deploy_function` - Upload WASM binary to plugins directory
- [ ] Implement `get_function_versions` - Track version history
- [ ] Implement `rollback_function` - Restore previous version
- [ ] Implement `delete_function` - Remove function
- [ ] Add file validation (check WASM magic bytes)
- [ ] Update `function_commands.rs` with new commands
- [ ] Update frontend bindings

## Technical Details

```rust
#[tauri::command]
pub async fn deploy_function(
    name: String,
    wasm_bytes: Vec<u8>,  // Base64 decoded
) -> Result<FunctionInfo, String> {
    // Validate WASM magic: 0x00 0x61 0x73 0x6D
    // Save to plugins/{name}/v{version}.wasm
    // Update manifest
}

#[tauri::command]
pub async fn get_function_versions(name: String) -> Result<Vec<FunctionVersion>, String> {
    // Read version history from manifest
}
```

## Acceptance Criteria

- [ ] Can upload .wasm files via UI
- [ ] Version history is maintained
- [ ] Can rollback to previous version
- [ ] Invalid WASM files are rejected

## Estimated Effort
4-5 hours
