use tauri::{AppHandle, Emitter};
use std::sync::{Arc, Mutex};
use serde::{Serialize, Deserialize};
use tokio::time::{sleep, Duration};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChaosExperiment {
    pub id: String,
    pub type_: String,
    pub target: String,
    pub intensity: u8,
    pub status: String,
    pub impact_score: u8,
}

pub struct ChaosState {
    pub experiments: Arc<Mutex<Vec<ChaosExperiment>>>,
}

#[tauri::command]
pub async fn get_experiments(state: tauri::State<'_, ChaosState>) -> Result<Vec<ChaosExperiment>, String> {
    let exps = state.experiments.lock().map_err(|e| e.to_string())?;
    Ok(exps.clone())
}

#[tauri::command]
pub async fn run_experiment(app: AppHandle, state: tauri::State<'_, ChaosState>, id: String) -> Result<(), String> {
    let mut exps = state.experiments.lock().map_err(|e| e.to_string())?;

    if let Some(exp) = exps.iter_mut().find(|e| e.id == id) {
        exp.status = "running".to_string();
        let exp_clone = exp.clone();

        // Emit event to frontend
        app.emit("chaos_update", exp_clone.clone()).map_err(|e| e.to_string())?;

        // Simulate experiment lifecycle in background
        let state_clone = state.experiments.clone();
        let app_clone = app.clone();
        let exp_id = id.clone();

        tauri::async_runtime::spawn(async move {
            sleep(Duration::from_secs(3)).await;

            // Healing phase
            {
                let mut exps = state_clone.lock().unwrap();
                if let Some(e) = exps.iter_mut().find(|e| e.id == exp_id) {
                    e.status = "healing".to_string();
                    e.impact_score = 75; // Mock score
                    app_clone.emit("chaos_update", e.clone()).unwrap();
                }
            }

            sleep(Duration::from_secs(3)).await;

            // Completed phase
            {
                let mut exps = state_clone.lock().unwrap();
                if let Some(e) = exps.iter_mut().find(|e| e.id == exp_id) {
                    e.status = "completed".to_string();
                    app_clone.emit("chaos_update", e.clone()).unwrap();
                }
            }
        });
    }

    Ok(())
}
