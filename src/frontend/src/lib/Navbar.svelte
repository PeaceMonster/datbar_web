<script lang='ts'>
    import NavbarElement from "./NavbarElement.svelte";
    import type { Page } from "./types";
    export let pages : Page[] = [];
</script>

<div class="navbar">
    {#each pages as page, i (i)}
        <NavbarElement page={page}/>
    {/each}
</div>


<style>
    .navbar {
        display: flex;
        width: 100%;
        margin-top: 10px;
        margin-bottom: 20px;
    }


</style>
