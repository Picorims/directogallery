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
        <div class="title-box">
            <h2>{title}</h2>
        </div>
        <h3>Directories</h3>
        {#each directories as dirName}
            <button>{dirName}</button>
        {/each}
        <h3>Files</h3>
        <div class="images-container">
            {#each files as file}
            <figure class="img-container">
                <img class="gallery-pic" src="{file.path}" alt="{(file.name === null)? "unknown" : file.name}">
            </figure>
            {/each}
        </div>
    </div>
{/if}

<style>
    div.container {
        padding: 20px;
        padding-bottom: 50px;
        box-sizing: border-box;
    }

    div.title-box {
        padding: 20px;
        background-color: var(--color-background-box);
        border-radius: 10px;
        box-shadow: 0 2px 4px var(--color-box-shadow);
    }

    h2 {
        font-size: 3rem;
        margin: 0;
        text-align: center;
        padding: 0.2em;
    }

    h3 {
        padding-bottom: 0.25em;
        font-size: 2rem;
        border-bottom: 2px solid var(--color-theme);
    }

    div.images-container {
        display: flex;
        flex-wrap: wrap;
        gap: 20px;
    }

    figure.img-container {
        flex: 1 0 auto;
        overflow: hidden;
        max-width: 100%;
        height: 200px;
        margin: 0;
        border-radius: 5px;
        box-shadow: 0 2px 4px var(--color-box-shadow);
    }

    img.gallery-pic {
        object-fit: cover;
        width: 100%;
        height: 100%;
    }
</style>