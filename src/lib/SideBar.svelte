<script>
    import { getDisks, setCurrentDirectory } from "./RustApi";
    import { listen } from "@tauri-apps/api/event";

    let disks = [["C:\\", "Local Disk(C:\\)"]];

    function updateDisks() {
        getDisks().then((message) => {
            disks = message;
        });
    }

    function loadDisk(path) {
        setCurrentDirectory(path);
    }

    listen("reload", (_) => updateDisks());

    updateDisks();
</script>

<div id="main_side">
    <div id="bord">
        <p>Disks</p>
        {#each disks as disk}
            <button class="disk_button" on:click={() => loadDisk(disk[0])}
                >{disk[1]}</button
            >
        {/each}
        <p>Favourites</p>
    </div>
</div>

<style>
    #bord {
        border-top: 8px solid #35ad5e;
    }

    #main_side {
        position: fixed;
        top: 32px;
        left: 0;

        height: calc(100% - 60px);
        width: 200px;

        display: flex;
        flex-direction: column;

        background-color: #444444;

        border-right: 1px #131313 solid;
    }
</style>
