#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use open::that;
use std::fs;
use std::path::Path;
use std::process::Command;
use std::thread;

#[tauri::command(rename_all = "snake_case")]
fn get_files(path_str: &str) -> Result<(Vec<(String, bool)>, &str), &str> {
    let current_path = Path::new(path_str);

    if !current_path.exists() {
        return Err("This path is invalid");
    }

    if current_path.is_dir() {
        if let Ok(path_unwrap) = fs::read_dir(current_path) {
            let output = path_unwrap
                .filter(|f| {
                    f.as_ref().unwrap().path().is_file() || f.as_ref().unwrap().path().is_dir()
                })
                .map(|f| {
                    (
                        String::from(f.as_ref().unwrap().path().to_str().unwrap()),
                        f.as_ref().unwrap().path().is_dir(),
                    )
                })
                .collect::<Vec<(String, bool)>>();
            return Ok((output, path_str));
        }

        return Err("Inaccesible dir!");
    }

    let _ = open_file(path_str);
    return get_files(current_path.parent().unwrap().to_str().unwrap());
}

#[tauri::command(rename_all = "snake_case")]
fn open_cmd(path_str: &str) {
    let pth = String::from(path_str);
    thread::spawn(|| {
        Command::new("cmd")
            .args(["/C", "start", "cmd"])
            .current_dir(pth)
            .output()
            .expect("REEE")
    });
}

#[tauri::command(rename_all = "snake_case")]
fn get_parent_dir(path_str: &str) -> String {
    let current_path = Path::new(path_str);
    if !current_path.exists() {
        return String::from("C:\\");
    }

    if let Some(parent) = current_path.parent() {
        return String::from(parent.to_str().unwrap());
    }

    return String::from(path_str);
}

fn open_file(file_path: &str) -> Result<(), Box<dyn std::error::Error>> {
    let path = Path::new(file_path);
    if path.exists() {
        // Try to open the file using the default application
        that(path)?;
        Ok(())
    } else {
        Err(From::from("File does not exist"))
    }
}

fn main() {
    tauri::Builder::default()
        .invoke_handler(tauri::generate_handler![
            get_files,
            open_cmd,
            get_parent_dir
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
