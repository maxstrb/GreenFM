<script>
  import { invoke } from "@tauri-apps/api/tauri";
  import { message } from "@tauri-apps/api/dialog";

  let files_in_current_folder = [];
  let current_path = "F:\\";

  function getFiles(path = current_path) {
    invoke("get_files", { path_str: path })
      .then((rs_output) => {
        current_path = rs_output[1];
        files_in_current_folder = rs_output[0];
      })
      .catch((err) => {
        message(err, { title: "Green FM", type: "error" });
      });
  }

  async function openInCmd(path = current_path) {
    await invoke("open_cmd", { path_str: path });
  }

  async function parentDir(path = current_path) {
    let output = "";

    await invoke("get_parent_dir", { path_str: path }).then((rs_output) => {
      output = rs_output;
    });

    return output;
  }

  getFiles();
</script>

<div>
  <p>Now in: {current_path}</p>
  <button
    on:click={() => {
      parentDir(current_path).then((rs_output) => {
        getFiles(rs_output);
      });
    }}>Back button</button
  >
  <div id="file_section">
    {#each files_in_current_folder as file}
      <div class="file_buttons">
        <button
          class="path_button"
          on:click={() => {
            getFiles(file[0]);
          }}>{file[0]}</button
        >
        {#if file[1]}
          <button on:click={() => openInCmd(file[0])}>cmd</button>
        {/if}
      </div>
    {/each}
  </div>
</div>

<style>
  #file_section {
    display: flex;
    flex-direction: column;
  }

  .file_buttons {
    display: flex;
  }

  .path_button {
    background-color: var(--primary);
  }
</style>
