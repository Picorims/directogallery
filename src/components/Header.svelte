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
    }

    h1 {
        margin: 0.5em 0;
        text-align: center;
        text-shadow: 0 2px 4px var(--color-text-shadow);
        color: var(--color-text-main-title);
    }
</style>