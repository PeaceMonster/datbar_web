<script lang='ts'>
    import type { Page } from "./types";
    export let pages : Page[] = [];
</script>

<div class="navbar">
    {#each pages as page, i (i)}
        <a href={page.route}>{page.name}</a>
    {/each}
    <a href="/admin">Admin</a>
</div>


<style>
    .navbar {
        display: flex;
        width: 100%;
        margin-top: 10px;
    }

    a {
        text-decoration: none;
        font-size: larger;
        display: inline-block;
        padding: 10px 20px;
        margin-bottom: 20px;
    }
</style>
