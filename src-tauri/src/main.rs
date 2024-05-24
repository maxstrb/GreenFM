#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]
use open::that;
use std::cmp::Ordering;
use std::fs;
use std::io;
use std::path::PathBuf;
use std::process::Command;
use std::sync::Mutex;
use std::thread;
use sysinfo::Disks;
use tauri::{Manager, State, Window};

use serde::{Deserialize, Serialize};
use serde_json::from_str;
use serde_json::to_string;
use std::fs::read_to_string;
use std::fs::File;
use std::io::Write;

struct CurrentDirectory(Mutex<PathBuf>);

#[derive(Serialize, Deserialize, Debug)]
struct FavouritePath(String, String);

fn save_to_file(vec: &Vec<FavouritePath>, filename: &str) {
    let json = to_string(vec).expect("Failed to serialize");
    let mut file = File::create(filename).unwrap();
    file.write_all(json.as_bytes());
}

fn load_from_file(filename: &str) -> Vec<(String, String)> {
    let json = read_to_string(filename).unwrap();
    let vec: Vec<FavouritePath> = from_str(&json).expect("Failed to deserialize");

    vec.into_iter().map(|f| (f.0, f.1)).collect()
}

fn remove_from_file<F>(filename: &str, predicate: F)
where
    F: Fn(&FavouritePath) -> bool,
{
    // Load the existing data
    let vec = load_from_file(filename);

    let mut vec_path = vec
        .iter()
        .map(|f| FavouritePath {
            0: f.clone().0,
            1: f.clone().1,
        })
        .collect::<Vec<FavouritePath>>();

    // Remove elements that match the predicate
    vec_path.retain(|x| !predicate(x));

    // Save the updated vector back to the file
    save_to_file(&vec_path, filename);
}

#[derive(Clone, serde::Serialize)]
struct Payload {
    message: String,
}

#[tauri::command(rename_all = "snake_case")]
fn get_current_directory(cur_dir: State<CurrentDirectory>) -> String {
    return cur_dir.0.lock().unwrap().to_str().unwrap().into();
}

#[tauri::command(rename_all = "snake_case")]
fn get_all_disks() -> Vec<(String, String)> {
    let disks = Disks::new_with_refreshed_list();

    disks
        .iter()
        .map(|disk| {
            let name_not_done = disk.name().to_str().unwrap();
            let path = disk.mount_point().to_str().unwrap();

            let name = if name_not_done == "" {
                "Local Disk"
            } else {
                &name_not_done
            };

            (path.to_string(), name.to_string() + " (" + path + ")")
        })
        .collect()
}

#[tauri::command(rename_all = "snake_case")]
fn set_current_directory(window: Window, cur_dir: State<CurrentDirectory>, new_path: &str) {
    let mut dir = cur_dir.0.lock().unwrap();
    let temp = PathBuf::from(new_path);

    if let Ok(ps) = check_path_state(&temp) {
        match ps {
            PathState::IsOpenDir => {
                *dir = temp.clone();
            }
            _ => {}
        }
    }

    let _ = window.emit("path_changed", Payload { message: "".into() });
}

#[tauri::command(rename_all = "snake_case")]
fn load_files_in_current_directory(
    cur_dir: State<CurrentDirectory>,
) -> Result<Vec<(String, String, bool)>, String> {
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
                                            files.push((
                                                file_name_str.to_string(),
                                                entry.path().to_str().unwrap().to_string(),
                                                false,
                                            ));
                                        }
                                    }
                                }
                                PathState::IsOpenDir => {
                                    if let Some(file_name) = path.file_name() {
                                        if let Some(file_name_str) = file_name.to_str() {
                                            files.push((
                                                file_name_str.to_string(),
                                                entry.path().to_str().unwrap().to_string(),
                                                true,
                                            ));
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

            files.sort_by(|a, b| {
                if a.2 == b.2 {
                    return Ordering::Equal;
                } else if a.2 {
                    return Ordering::Less;
                }

                return Ordering::Greater;
            });

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
fn get_ancestors(cur_dir: State<CurrentDirectory>) -> Vec<(String, String)> {
    let mut output: Vec<(String, String)> = (&(*cur_dir.0.lock().unwrap()))
        .ancestors()
        .map(|f| {
            (
                f.to_str().unwrap().to_string(),
                if let Some(name) = f.file_name() {
                    name.to_str().unwrap().to_string() + "\\"
                } else {
                    f.to_str().unwrap().to_string()
                },
            )
        })
        .collect();
    output.reverse();

    return output;
}

#[tauri::command(rename_all = "snake_case")]
fn get_parent_dir(cur_dir: State<CurrentDirectory>) -> String {
    let cur_dir_locked = cur_dir.0.lock().unwrap();
    let dir = cur_dir_locked.parent();

    if let Some(par) = dir {
        return par.to_str().unwrap().into();
    }

    cur_dir_locked.to_str().unwrap().to_string()
}

#[tauri::command(rename_all = "snake_case")]
fn open_file(file_path: &str) -> Result<(), String> {
    let dir = PathBuf::from(file_path);

    if let Ok(ps) = check_path_state(&dir) {
        if let PathState::IsFile = ps {
            // Try to open the file using the default application
            if let Ok(_) = that(dir.as_path()) {
                return Ok(());
            }

            return Err("File does not exit".to_string());
        }
    }

    Err("File does not exit".to_string())
}

#[derive(PartialEq, Debug)]
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
        .plugin(tauri_plugin_context_menu::init())
        .manage(CurrentDirectory(Mutex::new("C:\\".into())))
        .invoke_handler(tauri::generate_handler![
            load_files_in_current_directory,
            get_ancestors,
            get_parent_dir,
            open_file,
            set_current_directory,
            get_current_directory,
            open_cmd,
            get_all_disks,
        ])
        .setup(|app| {
            app.listen_global("open_cmd_from_current", move |event| {
                if let Some(payload) = event.payload() {
                    open_cmd(payload.trim_matches('"'));
                }
            });
            Ok(())
        })
        .run(tauri::generate_context!())
        .expect("error while running tauri application");
}
