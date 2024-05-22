<script>
    import { cli } from "@tauri-apps/api";
    import { setCurrentDirectory, parentDir } from "./RustApi.js";
    import { emit } from "@tauri-apps/api/event";
    let breadcrums = [];

    async function back() {
        await parentDir().then((message) => {
            setCurrentDirectory(message);
        });
    }

    function reload() {
        emit("path_changed");
    }
</script>

<div id="header_div">
    <div id="buttons">
        <button on:click={back}>↶</button>
        <button on:click={reload}>↻</button>
        <button>•••</button>
    </div>
</div>

<style>
    #buttons {
        display: flex;
        justify-content: space-around;
    }

    #header_div {
        top: 0;
        left: 0;
        position: fixed;
        display: flex;
        background-color: #2c392f;
        border-bottom: 1px black solid;
        width: 100%;
        height: 32px;
    }
</style>
