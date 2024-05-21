import { invoke } from "@tauri-apps/api/tauri";

export async function updateFilesInFolder() {
    let files = [];
    await invoke("load_files_in_current_directory").then((message) => {
        files = message;
    });

    return files;
}

export function changeCurrentDirectory(path) {
    invoke("change_current_directory", { new_path: path });
}

export function openFile(path) {
    invoke("open_file", { file_path: path })
}

export function setCurrentDirectory(path) {
    invoke("change_current_directory", { new_path: path });
}

export function openCommandLine(path) {
    invoke("open_cmd", { folder_path: path });
}

export async function getAncestors(){
    let ancestors = [];
    await invoke("get_ancerstos").then(
        (message) => {ancestors = message}
    )

    return ancestors;
}

export async function parentDir(){
    let parent = ["C:\\", "C"];
    await invoke("get_parent_dir").then(
        (message) => {parent = message}
    )

    return parent;
}