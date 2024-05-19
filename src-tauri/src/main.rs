#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use std::fs;
use std::process::Command;

#[tauri::command]
fn greet(name: &str) -> String {
    format!("Hello, {}! You've been greeted from Rust!", name)
}

#[tauri::command(rename_all = "snake_case")]
fn files(path_str: &str) -> Vec<String>{
    fs::read_dir(path_str).unwrap().map(|f| f.unwrap().path().display().to_string()).collect::<Vec<String>>()
}

#[tauri::command(rename_all = "snake_case")]
fn open_cmd(path_str: &str){
    Command::new("cmd")
        .args(["/C", "start", "cmd"])
        .current_dir(path_str)
        .output()
        .expect("failed to execute process");
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![greet, files, open_cmd])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
