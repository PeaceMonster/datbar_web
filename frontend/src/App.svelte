<script lang='ts'>
    import { onMount } from "svelte";
    import type { Page } from "./lib/types";
    import Forside from "./forside/Forside.svelte";
    import Barplan from "./barplan/Barplan.svelte";
    import Bartenders from "./bartenders/Bartenders.svelte";
    import Navbar from "./lib/Navbar.svelte";

    // Udlejning Subpages
    import Anlaeg from "./udlejning/Anlaeg.svelte";

    import Guides from "./guides/Guides.svelte";
    import P404 from "./lib/P404.svelte";
    import Events from "./events/Events.svelte";
    import About from "./about/About.svelte";
    import Bestyrelse from "./bestyrelse/Bestyrelse.svelte";


    let pages: Page[] = [
        {
            component : Forside,
            name : "Forside",
            route : "/",
            subitems: [],
        }, 
        {
            component : Barplan,
            name : "Barplan",
            route : "/barplan",
            subitems: [],
        },
        {
            component: Bartenders,
            name : "Bartendere",
            route : "/bartenders",
            subitems: [],
        },
        {
            component: P404,
            name : "Udlejning",
            route : "/udlejning",
            subitems : [
                {
                    name : "Udlejning af fadølsanlæg",
                    route : "/anlaeg",
                    component : Anlaeg
                }
            ]
        },
        {
            component: Guides,
            name: "Guides",
            route: "/guides",
            subitems: [],
        }, 
        {
            component: Bestyrelse,
            name: "Bestyrelsen",
            route: "/best",
            subitems : [],
        },
        {
            component: Events,
            name: "Events",
            route: "/events",
            subitems: [],
        },
        {
            component: About,
            name: "Om baren",
            route: "/about",
            subitems: [],
        },
        {
            component : P404,
            name : "Admin",
            route : "/admin/",
            subitems : []
        },
    ];
    
    let selected : ConstructorOfATypedSvelteComponent;
    let logo : string;
    onMount(() => {
        logo = window.location.origin + "/assets/logo.png";
        
        let url = window.location.pathname.split("/");
        let current_page = pages.find((page) => 
            page.route.split("/")[1] == url[1]
        );
        if (current_page == undefined) {
            selected = P404;
            return;
        }
        
        
        if (current_page?.subitems.length != 0 && url.length > 2 && url[2] != '') {
            
            let sub_current_page = current_page.subitems.find((page) => page.route.split("/")[1] == url[2]);
            
            if (sub_current_page == undefined) {
                selected = P404;
            } else {
                selected = sub_current_page.component;
            }
        } else {
            if (current_page.component != null) {
                selected = current_page.component;
            }
        }
        
    });
</script>

<main>
    <div class="header">
        <h1>
            <img src={logo} alt="" >
            Fredagscaféen
        </h1>
        <Navbar pages={pages}/>
    </div>

    <div class="container wrapper">
        <svelte:component this={selected}></svelte:component>
    </div>
    
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