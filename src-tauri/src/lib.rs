mod h_pt_engine;

use h_pt_engine::HptOperator;
use std::sync::Mutex;
use tauri::State;

pub struct HptState(pub Mutex<HptOperator>);

#[tauri::command]
pub fn generate_next_zero(state: State<HptState>) -> Result<f64, String> {
    let mut op = state.0.lock().map_err(|e| e.to_string())?;
    let guess = op.seed_state.iter().sum::<f64>() / op.seed_state.len() as f64;
    let guess = if guess.abs() < 1e-6 { 10.0 } else { guess };
    op.generate_zero(guess).ok_or("فشل التوليد".into())
}

#[tauri::command]
pub fn get_spectrum(state: State<HptState>) -> Vec<f64> {
    state.0.lock().unwrap().spectrum_buffer.clone()
}

#[tauri::command]
pub fn get_status_cmd() -> String {
    "Ismail Causal Engine — H_PT Online".to_string()
}

#[cfg_attr(mobile, tauri::mobile_entry_point)]
pub fn run() {
    tauri::Builder::default()
       .manage(HptState(Mutex::new(HptOperator::new())))
       .invoke_handler(tauri::generate_handler![
            generate_next_zero,
            get_spectrum,
            get_status_cmd
        ])
       .run(tauri::generate_context!())
       .expect("error while running tauri application");
}
