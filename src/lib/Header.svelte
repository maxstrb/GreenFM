<script>
    import {
        setCurrentDirectory,
        changeCurrentDirectory,
        openCommandLine,
        getAncestors,
        parentDir,
    } from "./RustApi.js";

    async function Back() {
        await parentDir().then((message) => {
            setCurrentDirectory(message);
        });
    }
</script>

<div class="header_div">
    <button on:click={Back}>Back</button>
</div>

<style>
    .header_div {
        display: flex;
    }
</style>
