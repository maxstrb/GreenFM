<script>
  import {
    updateFilesInFolder,
    setCurrentDirectory,
    openFile,
    showMenu,
  } from "./RustApi.js";

  import { listen, emit } from "@tauri-apps/api/event";

  let files_in_current_folder = [];

  let current_button = ["", "", false];

  function set_button(file) {
    current_button = file;
  }

  function reset_button(file) {
    if (current_button == file) {
      current_button = ["", "", false];
    }
  }

  function changeDirectory(path) {
    setCurrentDirectory(path);
  }

  listen("path_changed", (_) => {
    updateFiles();
  });

  listen("reload", (_) => {
    updateFiles();
  });

  function updateFiles() {
    updateFilesInFolder().then((message) => {
      files_in_current_folder = message;
    });
  }

  function createItems() {
    if (current_button[0] == null || current_button[0] == "") {
      return { items: [] };
    } else if (current_button[1]) {
      return {
        items: [
          {
            label: "Open in cmd",
            event: "open_cmd",
            payload: current_button[1],
          },
        ],
      };
    }

    return { items: [] };
  }

  window.addEventListener("contextmenu", async (e) => {
    showMenu(e, createItems());
  });

  listen("open_cmd", (event) => {
    emit("open_cmd_from_current", event.payload);
  });

  updateFiles();
</script>

<div id="files_main">
  {#each files_in_current_folder as file}
    {#if file[2]}
      <div class="center">
        <button
          class="folder_button"
          on:mouseleave={() => reset_button(file)}
          on:mouseenter={() => set_button(file)}
          on:click={() => changeDirectory(file[1])}
          ><img src="/folder.svg" alt="folder" width="16px" height="16px" />
          {file[0]}</button
        >
      </div>
    {/if}
    {#if !file[2]}
      <div class="center">
        <button
          class="file_button"
          on:mouseleave={() => reset_button(file)}
          on:mouseenter={() => set_button(file)}
          on:click={() => openFile(file[1])}
          ><img src="/file.svg" alt="folder" width="16px" height="16px" />
          {file[0]}</button
        >
      </div>
    {/if}
  {/each}
</div>

<style>
  #files_main {
    background-color: #2e2e2e;
    width: 100%;
  }

  .center {
    display: flex;
    justify-content: center;
  }

  .file_button {
    color: #ffffff;
    background-color: inherit;
    border: none;

    margin-top: 2px;
    margin-bottom: 2px;
    margin-left: 2px;

    padding: 0;
    width: calc(100% - 10px);

    text-align: left;
    font-size: 16px;

    cursor: pointer;
    display: inline-block;
  }

  .file_button:hover {
    background-color: #444444;
    border: 1px solid #595959;

    margin-top: 1px;
    margin-bottom: 1px;
    margin-left: 0px;
  }

  .folder_button {
    color: #ffffff;
    background-color: inherit;
    border: none;

    margin-top: 2px;
    margin-bottom: 2px;
    margin-left: 2px;

    padding: 0;
    width: calc(100% - 10px);

    text-align: left;
    font-size: 16px;

    cursor: pointer;
    display: inline-block;
  }

  .folder_button:hover {
    background-color: #444444;
    border: 1px solid #595959;

    margin-top: 1px;
    margin-bottom: 1px;
    margin-left: 0px;
  }
</style>
