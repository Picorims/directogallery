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
    import { open } from "@tauri-apps/api/dialog";
    import { invoke } from "@tauri-apps/api/tauri";
    import { loadCurrentDirJSON } from "src/stores";

    let selectedRoot: String = "No root selected.";

    /**
     * Opens the folder dialog to pick the directory being the root of the gallery to browse.
     * Loads and display the selected root.
     */
    async function selectRoot() {
        let selectedPath = await open({
            directory: true,
            multiple: false,
            recursive: true,
            title: "Choose the root directory of the gallery"
        });

        if (!Array.isArray(selectedPath) && selectedPath !== null) {
            selectedRoot = selectedPath;
            // Learn more about Tauri commands at https://tauri.app/v1/guides/features/command
            try {
                await invoke("cache_root", {path: selectedRoot});
                window.scrollTo(0, 0);
                await loadCurrentDirJSON(true);
            } catch (e) {
                alert("Could not cache the current directory.");
            }
        }
    }
</script>

<div class="container">
    <button on:click={selectRoot}>Choose root</button>
    <p class="root-chosen">{selectedRoot}</p>
</div>

<style>
    div.container {
        display: flex;
        align-items: center;
        text-align: center;
    }

    p.root-chosen {
        margin: 0 0.5em;
    }
</style>