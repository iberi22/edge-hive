use tauri::{State, AppHandle, Manager};
use std::sync::{Arc, Mutex};
use tokio::fs::File;
use tokio::io::{AsyncBufReadExt, BufReader, AsyncSeekExt};
use notify::{Watcher, RecursiveMode, RecommendedWatcher};
use std::path::PathBuf;
use tokio::sync::mpsc;

// Basic in-memory log store for demo purposes
// In a real app, this would query edge-hive-logging or similar
pub struct LogState {
    pub logs: Arc<Mutex<Vec<LogEntry>>>,
}

#[derive(serde::Serialize, Clone, Debug)]
pub struct LogEntry {
    pub id: String,
    pub timestamp: String,
    pub level: String,
    pub service: String,
    pub message: String,
}

#[tauri::command]
pub async fn get_logs(state: State<'_, LogState>) -> Result<Vec<LogEntry>, String> {
    let logs = state.logs.lock().map_err(|e| e.to_string())?;
    Ok(logs.clone())
}

#[tauri::command]
pub async fn add_log(state: State<'_, LogState>, level: String, message: String) -> Result<(), String> {
    let mut logs = state.logs.lock().map_err(|e| e.to_string())?;
    logs.push(LogEntry {
        id: uuid::Uuid::new_v4().to_string(),
        timestamp: chrono::Local::now().to_rfc3339(),
        level,
        service: "admin-ui".to_string(),
        message,
    });
    // Keep last 1000 logs
    if logs.len() > 1000 {
        logs.remove(0);
    }
    Ok(())
}

#[tauri::command]
pub async fn stream_logs(app_handle: AppHandle) -> Result<(), String> {
    let log_path = app_handle.path().app_log_dir()
        .ok_or_else(|| "Failed to resolve log directory".to_string())?
        .join("edge-hive-core.log");

    tauri::async_runtime::spawn(async move {
        let (tx, mut rx) = mpsc::channel(1);

        let mut watcher: RecommendedWatcher = notify::recommended_watcher(move |res| {
            if let Ok(event) = res {
                if let notify::EventKind::Modify(_) = event.kind {
                    tx.blocking_send(()).unwrap();
                }
            }
        }).unwrap();

        if let Err(e) = watcher.watch(&log_path, RecursiveMode::NonRecursive) {
            app_handle.emit("log-stream-error", format!("Failed to watch log file: {}", e)).unwrap();
            return;
        }

        let mut file = if let Ok(f) = File::open(&log_path).await {
            f
        } else {
            app_handle.emit("log-stream-error", "Failed to open log file").unwrap();
            return;
        };

        // Seek to the end of the file to only read new lines.
        file.seek(std::io::SeekFrom::End(0)).await.unwrap();
        let mut reader = BufReader::new(file);
        let mut line = String::new();

        loop {
            tokio::select! {
                Some(_) = rx.recv() => {
                    while let Ok(bytes_read) = reader.read_line(&mut line).await {
                        if bytes_read == 0 {
                            break;
                        }
                        app_handle.emit("log-message", line.clone()).unwrap();
                        line.clear();
                    }
                }
            }
        }
    });

    Ok(())
}
