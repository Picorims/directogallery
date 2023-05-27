// This file is part of Directogallery

// Directogallery is an application to browse a directory of images
// from the file system in a more convenient way.
// Copyright (C) 2023  Charly Schmidt alias Picorims<picorims.contact@gmail.com>

// Directogallery is free software: you can redistribute it and/or modify
// it under the terms of the GNU General Public License as published by
// the Free Software Foundation, either version 3 of the License, or
// (at your option) any later version.

// Directogallery is distributed in the hope that it will be useful,
// but WITHOUT ANY WARRANTY; without even the implied warranty of
// MERCHANTABILITY or FITNESS FOR A PARTICULAR PURPOSE.  See the
// GNU General Public License for more details.

// You should have received a copy of the GNU General Public License
// along with Directogallery.  If not, see <https://www.gnu.org/licenses/>.

import { writable, type Writable } from "svelte/store";
import { invoke } from "@tauri-apps/api/tauri";

export interface FileContent {
    name: string | null,
    path: string
}

export interface DirContent {
    name: string,
    path: string,
    files: Array<FileContent>,
    directories: Array<String>
}

/**
 * Information about the currently browsed directory
 */
export const currentDir: Writable<DirContent | null> = writable(null);
export const stack: Writable<Array<String>> = writable([]);
export const lockScroll: Writable<boolean> = writable(false);
export const imgScale: Writable<number> = writable(200);

/**
 * Loads the currently selected directory in the UI by requesting
 * its JSON and updating the store.
 */
export async function loadCurrentDirJSON(resetStack: boolean = false) {
    try {
        let newJSON: DirContent = await invoke("get_current_dir_data");
        if (resetStack) stack.set([newJSON.name]); // reset stack
        currentDir.set(newJSON);
    } catch (e) {
        alert("Could not read the current directory.");
    }
}
