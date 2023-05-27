<!--
This file is part of Directogallery

Directogallery is an application to browse a directory of images
from the file system in a more convenient way.
Copyright (C) 2023  Charly Schmidt alias Picorims<picorims.contact@gmail.com>

Directogallery is free software: you can redistribute it and/or modify
it under the terms of the GNU General Public License as published by
the Free Software Foundation, either version 3 of the License, or
(at your option) any later version.

Directogallery is distributed in the hope that it will be useful,
but WITHOUT ANY WARRANTY; without even the implied warranty of
MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
GNU General Public License for more details.

You should have received a copy of the GNU General Public License
along with Directogallery.  If not, see <https://www.gnu.org/licenses/>.
-->

<script lang="ts">
    import { currentDir, loadCurrentDirJSON, type FileContent, stack, lockScroll, imgScale } from "../stores";
    import {convertFileSrc, invoke} from "@tauri-apps/api/tauri"
    import ImageViewer from "./ImageViewer.svelte";

    let show: boolean = false;
    let showImg: boolean = false;
    let shownImgName: string = "";
    let shownImgPath: string = "";
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

    /**
     * Explores the child directory specified by `name`,
     * refresh the UI to load this child.
     * @param name name of the child directory
     */
    async function browseChild(name: String) {
        try {
            await invoke("navigate_to_child_dir", {name: name});
            $stack.push(name);
            $stack = $stack; // update state
        } catch (e) {
            alert("Could not read the child directory.");
        } finally {
            await loadCurrentDirJSON();
        }
    }

    /**
     * Explores the parent directory and refresh the UI.
     */
    async function browseParent(refresh: boolean = true) {
        try {
            await invoke("navigate_to_parent_dir");
            $stack.pop();
            $stack = $stack; // update state
        } catch (e) {
            alert("Could not read the parent directory: " + e);
        } finally {
            await loadCurrentDirJSON();
        }
    }

    /**
     * Go to the specified element of the navigation stack by going
     * up the parent tree.
     * @param i
     */
    async function browseStack(i: number) {
        if (i === $stack.length - 1) return; // can't browse to itself
        
        let top = $stack.length - 1;
        while (top > i) {
            browseParent(false);
            top--;
        }
        await loadCurrentDirJSON();
    }

    /**
     * Shows the image dialog for the given image metadata
     * @param path
     * @param name
     */
    function showImage(path: string, name: string) {        
        shownImgName = name;
        shownImgPath = path;
        showImg = true;
        lockScroll.set(true);
    }

    /**
     * hides the image dialog
     */
    function hideImage() {
        showImg = false;
        lockScroll.set(false);
    }
</script>

<div class="container">
    {#if show}
        <div class="stack-container g-box">
            <button on:click={() => browseParent()} disabled={$stack.length === 1}>Back</button>
            
            <div class="stack">
                {#each $stack as stackItem, i}
                    <!-- warning below handled conditionnally-->
                    <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
                    <span tabindex={i === ($stack.length-1)? null : 0}
                        class="stack-item"
                        class:locked={i === ($stack.length-1)}
                        role={i === ($stack.length-1)? null : "button"}
                        on:click={() => {browseStack(i)}}
                        on:keydown={e => {
                            if (e.key === "Enter") {
                                browseStack(i);
                            }
                        }}
                        >
                        {stackItem}
                    </span>
                    
                    {#if i !== ($stack.length - 1)}
                        <span class="stack-separator">&gt;</span>
                    {/if}
                {/each}
            </div>
        </div>
        
        <div class="title-box g-box">
            <h2>{title}</h2>
        </div>
        
        <h3>Directories</h3>
        {#each directories as dirName}
            <button on:click={e => browseChild(dirName)}>{dirName}</button>
        {/each}
        
        <h3>Files</h3>
        <div class="images-container">
            {#each files as file}
            <!-- svelte-ignore a11y-no-noninteractive-tabindex -->
            <figure class="img-container" tabindex="0" style="height: {$imgScale}px"
                on:click={() => showImage(file.path, file.name)}
                on:keydown={e => {
                    if (e.key === "Enter" || e.key === " ") {
                        showImage(file.path, file.name);
                    }
                }}
                >
                <img class="gallery-pic" src="{file.path}" alt="{(file.name === null)? "unknown" : file.name}">
            </figure>
            {/each}
        </div>
    {:else}
        <p class="notice">
            Copyright (C) 2023  Charly Schmidt alias Picorims (picorims.contact@gmail.com) - GPL-3.0-or-later license
        </p>
    {/if}
</div>


{#if showImg}
    <ImageViewer name={shownImgName} path={shownImgPath}
    on:close={() => {
        hideImage();
    }}
    />
{/if}

<style>
    p.notice {
        margin: 10px;
        text-align: center;
    }

    div.container {
        padding: 20px;
        padding-top: 70px;
        padding-bottom: 50px;
        box-sizing: border-box;
    }

    div.stack-container {
        display: flex;
        margin: 10px 0;
    }

    div.stack {
        display: flex;
        align-items: center;
        margin-left: 20px;
        font-weight: bold;
    }

    span.stack-item {
        font-size: 1.25rem;
        color: var(--color-theme);
    }
    span.stack-item.locked {
        color: var(--color-faded-content);
    }

    span.stack-item:hover:not(.locked),
    span.stack-item:focus:not(.locked) {
        cursor: pointer;
        color: var(--color-theme-contrast);
        text-decoration: underline;
    }

    span.stack-separator {
        font-size: 1.5rem;
        margin: 0 0.5em;
        color: var(--color-faded-content);
    }

    div.title-box {
        padding: 20px;
    }

    h2 {
        font-size: 3rem;
        margin: 0;
        text-align: center;
        padding: 0.2em;
        text-shadow: 0 2px 4px var(--color-text-shadow);
        color: var(--color-text-main-title);
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
        transition: 0.1s;
    }

    figure.img-container:hover, figure.img-container:focus {
        transform: scale(1.02);
        cursor: pointer;
        opacity: 0.8;
    }

    img.gallery-pic {
        object-fit: cover;
        width: 100%;
        height: 100%;
    }
</style>