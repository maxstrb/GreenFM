<script>
  import Files from "./lib/Files.svelte";
  import Header from "./lib/Header.svelte";
  import SideBar from "./lib/SideBar.svelte";
</script>

<main class="container">
  <Header />

  <div id="bottom_part">
    <SideBar />
    <Files />
  </div>
</main>

<style>
  #bottom_part {
    display: flex;
  }
</style>
