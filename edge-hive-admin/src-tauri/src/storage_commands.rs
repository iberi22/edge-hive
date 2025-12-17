use tauri::State;
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};

pub struct StorageState {
    // In real app, this would be S3 client or VFS
    pub root_dir: std::path::PathBuf,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct BucketDTO {
    pub id: String,
    pub name: String,
    pub public: bool,
    pub size: String,
    pub files_count: u32,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct FileDTO {
    pub id: String,
    pub name: String,
    pub size: String,
    pub type_: String,
    pub last_modified: String,
}

#[tauri::command]
pub async fn list_buckets(state: State<'_, StorageState>) -> Result<Vec<BucketDTO>, String> {
    // access state securely
    // For demo, return mock or list actual folders in app_data_dir/storage
    Ok(vec![
        BucketDTO {
            id: "b1".into(),
            name: "public-assets".into(),
            public: true,
            size: "12 MB".into(),
            files_count: 5,
        },
        BucketDTO {
            id: "b2".into(),
            name: "private-backups".into(),
            public: false,
            size: "1.2 GB".into(),
            files_count: 12,
        }
    ])
}

#[tauri::command]
pub async fn list_files(state: State<'_, StorageState>, bucket_id: String) -> Result<Vec<FileDTO>, String> {
    Ok(vec![
        FileDTO {
            id: "f1".into(),
            name: "logo.png".into(),
            size: "2 MB".into(),
            type_: "image/png".into(),
            last_modified: "2024-01-01".into(),
        }
    ])
}
