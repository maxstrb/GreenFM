<script>
  import {
    updateFilesInFolder,
    changeCurrentDirectory,
    openFile,
    openCommandLine,
  } from "./RustApi.js";

  import { listen } from "@tauri-apps/api/event";

  let files_in_current_folder = [];

  function changeDirectory(path) {
    changeCurrentDirectory(path);
  }

  //#region Updating files
  const _filesChanged = listen("path_changed", (_) => {
    updateFiles();
  });

  function updateFiles() {
    updateFilesInFolder().then((message) => {
      files_in_current_folder = message;
    });
  }

  updateFiles();
  //#endregion
</script>

<div id="files_main">
  {#each files_in_current_folder as file}
    {#if file[1]}
      <div class="center">
        <button class="folder_button" on:click={() => changeDirectory(file[0])}
          >{file[0]}</button
        >
      </div>
    {/if}
    {#if !file[1]}
      <div class="center">
        <button class="file_button" on:click={() => openFile(file[0])}
          >{file[0]}</button
        >
      </div>
    {/if}
  {/each}
</div>

<style>
  #files_main {
    background-color: #444444;
  }

  .center {
    display: flex;
    justify-content: center;
  }

  .folder_button {
    text-align: left;
    width: 90%;
    border: none;
    background-color: inherit;
    padding: 14px 28px;
    font-size: 16px;
    cursor: pointer;
    display: inline-block;
  }
</style>
