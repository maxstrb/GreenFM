<script>
    import {
        setCurrentDirectory,
        getAncestors,
        getCurrentDirectory,
        openCmd,
    } from "./RustApi.js";
    import { listen } from "@tauri-apps/api/event";

    function updateBreadcrums() {
        getAncestors().then((anc) => {
            breadcrums = anc;
        });
    }

    let breadcrums = [];

    function setBread(crum) {
        setCurrentDirectory(crum);
    }

    function open_cmd_here() {
        getCurrentDirectory().then((cd) => {
            openCmd(cd);
        });
    }

    listen("path_changed", (_) => {
        updateBreadcrums();
    });
    updateBreadcrums();
</script>

<div id="main_footer">
    <div id="breadcrums">
        {#each breadcrums as crum}
            <button class="bread" on:click={() => setBread(crum[0])}
                >{crum[1]}</button
            >
        {/each}
    </div>
    <button id="open_cmd" on:click={open_cmd_here}>~</button>
</div>

<style>
    #main_footer {
        position: fixed;
        bottom: 0;
        right: 0;

        width: 100%;
        height: 28px;
        z-index: 1;

        background-color: #505050;

        display: flex;
        justify-content: space-between;
        justify-items: center;
    }

    #breadcrums {
        display: flex;
        margin: 5px;
    }

    #open_cmd {
        width: 24px;
        height: 24px;
        text-align: center;
        background-color: #9ddb9f;

        margin-right: 2px;
        margin-top: 2px;

        font-size: large;

        border: none;
        border-radius: 5px;
    }

    .bread {
        border: none;
        padding: 0;
        margin: 0;
        cursor: pointer;
        background-color: inherit;
        color: white;
    }

    .bread:hover {
        text-decoration: underline;
    }
</style>
