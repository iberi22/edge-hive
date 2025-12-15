use portable_pty::{native_pty_system, CommandBuilder, PtySize};
use std::io::{BufRead, BufReader, Write};
use std::sync::{Arc, Mutex};
use std::thread;
use tauri::{AppHandle, Manager};

#[derive(Clone, serde::Serialize)]
struct TerminalOutput {
    data: String,
}

pub struct TerminalState {
    pub writer: Arc<Mutex<Option<Box<dyn Write + Send>>>>,
}

#[tauri::command]
pub async fn terminal_spawn(app: AppHandle) -> Result<String, String> {
    let pty_system = native_pty_system();

    // Create PTY with size
    let pair = pty_system
        .openpty(PtySize {
            rows: 24,
            cols: 80,
            pixel_width: 0,
            pixel_height: 0,
        })
        .map_err(|e| format!("Failed to open PTY: {}", e))?;

    // Spawn shell (cross-platform)
    let shell = if cfg!(target_os = "windows") {
        CommandBuilder::new("powershell.exe")
    } else {
        CommandBuilder::new("sh")
    };

    let mut child = pair
        .slave
        .spawn_command(shell)
        .map_err(|e| format!("Failed to spawn shell: {}", e))?;

    // Store writer for input
    let writer = pair.master.take_writer().map_err(|e| e.to_string())?;
    let state = app.state::<TerminalState>();
    *state.writer.lock().unwrap() = Some(writer);

    // Read output in background thread
    let reader = pair.master.try_clone_reader().map_err(|e| e.to_string())?;
    let app_handle = app.clone();

    thread::spawn(move || {
        let mut buf_reader = BufReader::new(reader);
        let mut buffer = String::new();

        loop {
            buffer.clear();
            match buf_reader.read_line(&mut buffer) {
                Ok(0) => break, // EOF
                Ok(_) => {
                    // Emit output to frontend
                    let _ = app_handle.emit("terminal-output", TerminalOutput {
                        data: buffer.clone(),
                    });
                }
                Err(e) => {
                    eprintln!("Terminal read error: {}", e);
                    break;
                }
            }
        }

        // Wait for child process
        let _ = child.wait();
    });

    Ok("Terminal spawned".to_string())
}

#[tauri::command]
pub async fn terminal_write(
    state: tauri::State<'_, TerminalState>,
    data: String,
) -> Result<(), String> {
    let mut writer_guard = state.writer.lock().unwrap();
    if let Some(writer) = writer_guard.as_mut() {
        writer
            .write_all(data.as_bytes())
            .map_err(|e| format!("Write error: {}", e))?;
        writer.flush().map_err(|e| format!("Flush error: {}", e))?;
        Ok(())
    } else {
        Err("Terminal not initialized".to_string())
    }
}

#[tauri::command]
pub async fn terminal_resize(rows: u16, cols: u16) -> Result<(), String> {
    // TODO: Store PTY handle and resize
    // For now, just acknowledge
    println!("Terminal resize: {}x{}", rows, cols);
    Ok(())
}
