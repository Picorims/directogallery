<script lang="ts">
    import { currentDir, type FileContent } from "../stores";
    import {convertFileSrc} from "@tauri-apps/api/tauri"

    let show: boolean = false;
    let title: string;
    let directories: Array<String> = [];
    let files: Array<FileContent>;

    currentDir.subscribe((dir) => {
        show = (dir !== null);
        if (show) {
            title = dir.name;
            directories = dir.directories;
            files = dir.files;
            // required to load local assets inside the picked directory,
            // which is added to the assetScope by Tauri.
            files.map(file => {file.path = convertFileSrc(file.path)});
        }
    });

</script>

{#if show}
    <div class="container">
        <h1>{title}</h1>
        <h2>Directories</h2>
        {#each directories as dirName}
            <button>{dirName}</button>
        {/each}
        <h2>Files</h2>
        {#each files as file}
            <img src="{file.path}" alt="{(file.name === null)? "unknown" : file.name}">
        {/each}
    </div>
{/if}

<style>
    div.container {
        padding: 20px;
    }
</style>