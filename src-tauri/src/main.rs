#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use open::that;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use std::thread;
use tauri::{Manager, State, Window};

struct CurrentDirectory(Mutex<PathBuf>);

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command(rename_all = "snake_case")]
fn change_current_directory(window: Window, cur_dir: State<CurrentDirectory>, new_path: &str) {
    let mut dir = cur_dir.0.lock().unwrap();
    let temp = dir.join(PathBuf::from(new_path));

    if let Ok(ps) = check_path_state(&temp) {
        match ps {
            PathState::IsOpenDir => {
                *dir = temp.clone();
                let _ = window.emit(
                    "path_changed",
                    Payload {
                        message: temp.to_str().unwrap().into(),
                    },
                );
            }
            _ => {}
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
fn set_current_directory(window: Window, cur_dir: State<CurrentDirectory>, new_path: &str) {
    let mut dir = cur_dir.0.lock().unwrap();
    let temp = PathBuf::from(new_path);

    if let Ok(ps) = check_path_state(&temp) {
        match ps {
            PathState::IsOpenDir => {
                *dir = temp.clone();
                let _ = window.emit(
                    "path_changed",
                    Payload {
                        message: temp.to_str().unwrap().into(),
                    },
                );
            }
            _ => {}
        }
    }
}

#[tauri::command(rename_all = "snake_case")]
fn load_files_in_current_directory(
    cur_dir: State<CurrentDirectory>,
) -> Result<Vec<(String, bool)>, String> {
    let cd = &(*cur_dir.0.lock().unwrap());

    match fs::read_dir(cd) {
        Ok(entries) => {
            let mut files = Vec::new();
            for entry in entries {
                match entry {
                    Ok(entry) => {
                        let path = entry.path();
                        let path_state = check_path_state(&path);

                        if let Ok(ps) = path_state {
                            match ps {
                                PathState::IsFile => {
                                    if let Some(file_name) = path.file_name() {
                                        if let Some(file_name_str) = file_name.to_str() {
                                            files.push((file_name_str.to_string(), false));
                                        }
                                    }
                                }
                                PathState::IsOpenDir => {
                                    if let Some(file_name) = path.file_name() {
                                        if let Some(file_name_str) = file_name.to_str() {
                                            files.push((file_name_str.to_string(), true));
                                        }
                                    }
                                }

                                _ => {}
                            }
                        }
                    }
                    Err(e) => return Err(format!("Failed to read entry: {}", e)),
                }
            }
            Ok(files)
        }
        Err(e) => Err(format!("Failed to read directory: {}", e)),
    }
}

#[tauri::command(rename_all = "snake_case")]
fn open_cmd(folder_path: &str) {
    let pth = String::from(folder_path);
    thread::spawn(|| {
        Command::new("cmd")
            .args(["/C", "start", "cmd"])
            .current_dir(pth)
            .output()
            .expect("REEE")
    });
}

#[tauri::command(rename_all = "snake_case")]
fn get_ancestors(cur_dir: State<CurrentDirectory>) -> Vec<String> {
    (&(*cur_dir.0.lock().unwrap()))
        .ancestors()
        .map(|f| f.to_str().unwrap().to_string())
        .collect()
}

#[tauri::command(rename_all = "snake_case")]
fn get_parent_dir(cur_dir: State<CurrentDirectory>) -> String {
    let dir = get_ancestors(cur_dir.clone());
    if dir.len() >= 2 {
        return dir[1].clone();
    }

    let cd = &(*cur_dir.0.lock().unwrap());
    cd.to_str().unwrap().to_string()
}

#[tauri::command(rename_all = "snake_case")]
fn open_file(cur_dir: State<CurrentDirectory>, file_path: &str) -> Result<(), String> {
    let dir = cur_dir.0.lock().unwrap();
    let file_absolute_path = dir.join(PathBuf::from(file_path));

    if let Ok(ps) = check_path_state(&file_absolute_path) {
        if let PathState::IsFile = ps {
            // Try to open the file using the default application
            if let Ok(_) = that(file_absolute_path.as_path()) {
                return Ok(());
            }

            return Err("File does not exit".to_string());
        }
    }

    Err("File does not exit".to_string())
}

enum PathState {
    IsClosedDir,
    IsOpenDir,
    IsFile,
    DoesntExist,
}

fn check_path_state(path: &PathBuf) -> Result<PathState, ()> {
    match fs::metadata(path) {
        Ok(metadata) => {
            if metadata.is_dir() {
                match fs::read_dir(&path) {
                    Ok(_) => Ok(PathState::IsOpenDir), // The directory exists and is accessible
                    Err(e) => {
                        if e.kind() == io::ErrorKind::PermissionDenied {
                            Ok(PathState::IsClosedDir) // The directory exists but is not accessible due to permissions
                        } else {
                            Err(()) // Other types of errors
                        }
                    }
                }
            } else {
                Ok(PathState::IsFile) // The path exists but is not a directory
            }
        }
        Err(e) => {
            if e.kind() == io::ErrorKind::NotFound {
                Ok(PathState::DoesntExist) // The directory does not exist
            } else {
                Err(()) // Other types of errors
            }
        }
    }
}

fn main() {
    tauri::Builder::default()
        .manage(CurrentDirectory(Mutex::new("C:\\".into())))
        .setup(|app| {
            // `main` here is the window label; it is defined on the window creation or under `tauri.conf.json`
            // the default value is `main`. note that it must be unique
            let main_window = app.get_window("main").unwrap();

            main_window
                .emit(
                    "path_changed",
                    Payload {
                        message: "C:\\".into(),
                    },
                )
                .unwrap();
            Ok(())
        })
        .invoke_handler(tauri::generate_handler![
            load_files_in_current_directory,
            get_ancestors,
            open_cmd,
            get_parent_dir,
            open_file,
            change_current_directory,
            set_current_directory,
        ])
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
