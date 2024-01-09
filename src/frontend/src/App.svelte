<script lang='ts'>
    import { SvelteComponent, onMount } from "svelte";
    import type { Page } from "./lib/types";
    import Forside from "./forside/Forside.svelte";
    import Barplan from "./barplan/Barplan.svelte";
    import Bartenders from "./bartenders/Bartenders.svelte";

    let pages: Page[] = [
        {
            component : Forside,
            name : "Forside",
            route : "/"
        }, 
        {
            component : Barplan,
            name : "Barplan",
            route : "/barplan"
        },
        {
            component: Bartenders,
            name : "Bartenders",
            route : "/bartenders"
        }
    ];
    
    let selected : number = 0;

    onMount(() => {
        let url = window.location.pathname;
        selected = pages.findIndex((page) => page.route == url);
    });
</script>

<main>
    <div class="header">
        <h1>
            <img src="assets/logo.png" alt="" >
            Fredagscaféen
        </h1>
        <div class="navbar">
            {#each pages as page, i (i)}
                <a href={page.route}>{page.name}</a>
            {/each}
            <a href="/admin">Admin</a>
        </div>
    </div>

    <svelte:component this={pages[selected].component}></svelte:component>

    <footer>
        <hr>
        <p>Fredagscaféen er en fredagsbar for datalogi og IT på Aarhus Universitet.</p>
        <p>1993-2024 Fredagscaféen</p>
    </footer>

</main>


<style>


    h1 {
        color: midnightblue;
        font-family: serif;
        display: inline-block;
    }

    .navbar {
        display: flex;
        width: 100%;
        margin-top: 10px;
    }

    .header a {
        text-decoration: none;
        font-size: larger;
        display: inline-block;
        padding: 10px 20px;
        margin-bottom: 20px;
    }

    img {
        height: 50px;
        display: inline-block;
    }

    footer {
        text-align: center;
        margin: 20px;
    }

    footer p {
        margin: 0;
    }
</style>