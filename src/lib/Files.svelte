<script>
  import { invoke } from "@tauri-apps/api/tauri";

  let files = [];
  const default_path = "F:\\";

  let current_path = default_path;

  function goToPath(path = default_path) {
    invoke("files", { path_str: path }).then((message) => {
      current_path = path;
      files = message;
      console.log(path);
    });
  }

  function cmd(path = current_path) {
    invoke("open_cmd", { path_str: path });
  }

  goToPath();
</script>

<div>
  <p>{current_path}</p>
  <p>{current_path.split("\\").slice(0, -1).join("\\") + "\\"}</p>
  <button
    class="cmd_button"
    on:click={() => {
      cmd(current_path);
    }}>Current cmd</button
  >
  <button
    class="primary_button"
    on:click={() =>
      goToPath(current_path.split("\\").slice(0, -1).join("\\")) + "\\"}
    >Previous folder</button
  >

  <div id="files">
    {#each files as f}
      <div class="display_buttons">
        <button class="primary_button" on:click={() => goToPath(f)}>{f}</button>
        <button
          class="cmd_button"
          on:click={() => {
            cmd(f);
          }}>cmd</button
        >
      </div>
    {/each}
  </div>
</div>

<style>
  #files {
    display: flex;
    flex-direction: column;
  }

  .display_buttons {
    display: flex;
  }

  .primary_button {
    background-color: var(--primary);
  }
</style>
