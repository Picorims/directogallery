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
    let scrollY = 0;

    function backToTop() {
        window.scrollTo(0, 0);
    }
</script>

<button class:hidden={scrollY < 50} on:click={backToTop}>▲</button>

<svelte:window bind:scrollY={scrollY}/>

<style>
    button {
        position: fixed;
        bottom: 10px;
        right: 10px;
        padding: 0.2em;
        font-size: 2rem;
    }

    button.hidden {
        bottom: -100px;
    }
</style>