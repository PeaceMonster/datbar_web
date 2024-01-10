<script lang='ts'>
    import { SvelteComponent, onMount } from "svelte";
    import type { Page } from "./lib/types";
    import Forside from "./forside/Forside.svelte";
    import Barplan from "./barplan/Barplan.svelte";
    import Bartenders from "./bartenders/Bartenders.svelte";
    import Navbar from "./lib/Navbar.svelte";


    let pages: Page[] = [
        {
            component : Forside,
            name : "Forside",
            route : "/",
            menu: false,
            subitems: null,
        }, 
        {
            component : Barplan,
            name : "Barplan",
            route : "/barplan",
            menu: false,
            subitems: null,
        },
        {
            component: Bartenders,
            name : "Bartenders",
            route : "/bartenders",
            menu: false,
            subitems: null,
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
        <Navbar pages={pages} />
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