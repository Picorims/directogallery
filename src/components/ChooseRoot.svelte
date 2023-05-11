<script lang="ts">
    import { open } from "@tauri-apps/api/dialog";
    import { invoke } from "@tauri-apps/api/tauri";

    let selectedRoot: String = "No root selected.";

    /**
     * Opens the folder dialog to pick the directory being the root of the gallery to browse.
     */
    async function selectRoot() {
        // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
        // selectedRoot = await invoke("select_root");
        let selectedPath = await open({
            directory: true
        });

        if (!Array.isArray(selectedPath) && selectedPath !== null) {
            selectedRoot = selectedPath;
        }
    }
</script>

<div class="container">
    <div class="row">
        <button on:click={selectRoot}>Choose root</button>
    </div>
    <p class="root-chosen">{selectedRoot}</p>
</div>

<style>
    div.container {
        text-align: center;
        padding: 0 10px;
    }

    p.root-chosen {
        margin: 0.25em 0;
    }
</style>