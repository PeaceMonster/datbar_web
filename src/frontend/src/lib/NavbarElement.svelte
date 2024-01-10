<script lang='ts'>
    import type { Page } from "./types";
    export let page : Page;
    
    
</script>
{#if !page.menu }
    <a href={page.route}>{page.name}</a>
{/if}