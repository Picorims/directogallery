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
    import { createEventDispatcher } from "svelte";

    export let path: string = "";
    export let name: string = "";

    // lock controlled by the parent, necessary
    // if the parent has an enter event for the dialog
    // not to close immediately (The Enter key is still pressed
    // when the dialog shows up and is immediately handled by
    // the back button's onkeydown event).
    let lockBack: boolean = true;
    let disableBack: boolean = false;
    let lastFocusedElement: HTMLElement = null;

    const dispatch = createEventDispatcher();

    function sendClose() {
        if (!lockBack) {
            dispatch("close");
            disableBack = true;
            setTimeout(() => {lastFocusedElement.focus();}, 250);
        }
    }

    function focusBtn(el) {
        el.focus();
    }

    function saveLastFocused(el) {
        lastFocusedElement = document.activeElement as HTMLElement;
        console.log(lastFocusedElement);
        focusBtn(el);
    }
</script>

<div class="container">
    <button disabled={disableBack} id="image-viewer-back" class="back-btn" use:saveLastFocused
    on:keydown={(e) =>{ /*tab navigation locked to this button*/
        if (e.key === "Tab") {
            e.preventDefault();
        } else if (e.key === "Escape") {
            sendClose();
        }
    }}
    on:keyup={e => lockBack = false}
    on:mouseup={e => lockBack = false}
    on:focusout={e => {focusBtn(e.target) /*tab navigation locked to this button*/}}
    on:click={sendClose}
    >Back</button>
    <figure class="fig-container">
        <div class="img-container">
            <img class="" src={path} alt={name}>
        </div>
        <figcaption>{name}</figcaption>
    </figure>
</div>

<style>
    div.container {
        position: fixed;
        z-index: 100;
        display: flex;
        align-items: center;
        justify-content: center;
        top: 0;
        left: 0;
        width: 100vw;
        height: 100vh;
        background-color: #111111dd;
        overflow: hidden;
    }

    button.back-btn {
        position: absolute;
        top: 10px;
        right: 10px;
        box-shadow: 0 2px 4px var(--color-box-shadow-invert);
    }

    figure.fig-container {
        display: flex;
        flex-direction: column;
        align-items: center;
        justify-content: center;
        width: 100%;
        height: 100%;
        margin: 20px;
        color: var(--color-text-invert);
        font-weight: bold;
        text-align: center;
    }

    @media (prefers-color-scheme: dark) {
        button.back-btn {
            box-shadow: 0 2px 4px var(--color-box-shadow);
        }

        figure.fig-container {
            color: var(--color-text);
        }
    }

    figure.fig-container > figcaption {
        width: 100%;
    }
    
    div.img-container {
        width: 88%;
        height: 90%;
        margin: 0;
        overflow: hidden;
        margin-bottom: 10px;
    }

    div.img-container > img {
        object-fit: contain;
        width: 100%;
        height: 100%;
    }
</style>