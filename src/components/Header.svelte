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

<script>
    import ChooseRoot from "./ChooseRoot.svelte";
</script>

<div class="container">
    <h1>Directogallery</h1>
    <ChooseRoot />
</div>

<style>
    div.container {
        width: 100%;
        padding: 5px 20px;
        background-color: var(--color-background-box);
        overflow: auto; /*https://stackoverflow.com/questions/1762539/margin-on-child-element-moves-parent-element*/
        box-shadow: 0 2px 4px var(--color-box-shadow);
    }

    h1 {
        margin: 0.5em 0;
        text-align: center;
        text-shadow: 0 2px 4px var(--color-text-shadow);
        color: var(--color-text-main-title);
    }
</style>