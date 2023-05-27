<script lang="ts">
    let scrollY = 0;

    function backToTop() {
        window.scrollTo(0, 0);
    }
</script>

<button class:hidden={scrollY < 50} on:click={backToTop}>^</button>

<svelte:window bind:scrollY={scrollY}/>

<style>
    button {
        position: fixed;
        bottom: 10px;
        right: 10px;
    }

    button.hidden {
        bottom: -100px;
    }
</style>