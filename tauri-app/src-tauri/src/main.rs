#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use anyhow::Result;
use tauri::Manager;

#[tauri::command]
fn run_starlark(script: String) -> Result<String, String> {
    match black_anvil::run_starlark(&script) {
        Ok(s) => Ok(s),
        Err(e) => Err(e.to_string()),
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![run_starlark])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
