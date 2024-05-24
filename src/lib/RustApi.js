import { invoke } from "@tauri-apps/api/tauri";

export async function updateFilesInFolder() {
    let files = [];
    await invoke("load_files_in_current_directory").then((message) => {
        files = message;
    });

    return files;
}

export function openFile(path) {
    invoke("open_file", { file_path: path })
}

export function openCmd(path) {
    invoke("open_cmd", { folder_path: path })
}

export function setCurrentDirectory(path) {
    invoke("set_current_directory", { new_path: path });
}

export async function getAncestors(){
    let anc = ["C:\\"]
    await invoke("get_ancestors").then(
        (message) => {anc = message;}
    )

    return anc;
}

export async function parentDir(){
    let parent = "C:\\";
    await invoke("get_parent_dir").then(
        (message) => {parent = message}
    )

    return parent;
}

export async function getCurrentDirectory(){
    let cd = "C:\\";
    await invoke("get_current_directory").then(
        (message) => {cd = message;}
    )

    return cd;
}

export async function showMenu(e, items){
    e.preventDefault(); 
    invoke("plugin:context_menu|show_context_menu", items);
}

export async function getDisks(){
    let disks = [["C:\\", "Local Disk(C:\\)"]];
    await invoke("get_all_disks").then(
        (message) => {disks = message;}
    )

    return disks;
}