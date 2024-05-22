<script>
    import { setCurrentDirectory, getAncestors, parentDir } from "./RustApi.js";
    import { listen } from "@tauri-apps/api/event";

    let breadcrums = [];

    async function Back() {
        await parentDir().then((message) => {
            setCurrentDirectory(message);
        });
    }

    function updateBreadcrums() {
        getAncestors().then((anc) => {
            breadcrums = anc;
        });
    }

    function setBread(crum) {
        setCurrentDirectory(crum[0]);
    }

    listen("path_changed", (_) => {
        updateBreadcrums();
    });
    updateBreadcrums();
</script>

<div id="header_div">
    <button on:click={Back}>↶</button>
    <div id="breadcrums">
        {#each breadcrums as crum}
            <button class="bread" on:click={() => setBread(crum)}
                >{crum[1]}</button
            >
        {/each}
    </div>
</div>

<style>
    #breadcrums {
        display: flex;
    }

    .bread {
        border: none;
        padding: 0;
        margin: 0;
    }

    #header_div {
        top: 0;
        left: 0;
        position: fixed;
        display: flex;
        background-color: #2c392f;
        border-bottom: 1px black solid;
        width: 100vw;
        height: 28px;
    }
</style>
