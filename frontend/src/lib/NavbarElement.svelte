<script lang='ts'>
    import P404 from "./P404.svelte";
    import type { Page } from "./types";
    export let page : Page = {
        name: "Test",
        route: "/",
        component: P404,
        subitems : [{
                name: "Test",
                route: "/",
                component: P404,
            },
        ]
    };
    
    let drop = false;

    function toggle_dropdown() {
        drop = !drop;
    }
    
</script>
<div class="wrapper">
    {#if page.subitems.length == 0 }
        <a href={page.route}>{page.name}</a>
    {:else}
        <a href={page.route} on:click|preventDefault={toggle_dropdown} class:dropmenu={drop}>
            {page.name}
            <span class="caret"></span>
        </a>

        
        <div id="drop" hidden={!drop} class:drop>
            {#each page.subitems as sub}
                <a href={page.route + sub.route} class="subitem">{sub.name}</a>
            {/each}
        </div>
    {/if}
</div>


<style>
    a {
        text-decoration: none;
        font-size: larger;
        display: inline-block;
        padding: 10px 20px;
        
    }

    a:hover {
        background-color: #EEEEEE;   
    }

    a.dropmenu {
        background-color: #EEEEEE;   
    }

    a.subitem {
        padding: 0 10px;
        font-size: medium;
    }


    .drop {
        position: absolute;
        display: flex;
        flex-direction: column;
        gap: 10px;
        background-color: #FFFFFF;
        width: fit-content;
        padding: 5px 0;
        border: #EEEEEE solid 1px;
        box-shadow: #aaaaaa 0 0 10px;
        z-index: 1000;
    }
</style>