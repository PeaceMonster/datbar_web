<script lang='ts'>
    import Forside from "./forside/Forside.svelte";
    import Barplan from "./barplan/Barplan.svelte";

    let pages: any = [
        [Forside, "Forside"], 
        [Barplan, "Barplan"],
    ];
    let selected : number = 0;
</script>

<main>
    <div class="header">
        <h1>
            <img src="logo.png" alt="" >
            Fredagscaféen
        </h1>
        <div class="navbar">
            {#each pages as page, i (i)}
                <a href="/" on:click|preventDefault={() => selected = i}>{page[1]}</a>
            {/each}
            
        </div>
    </div>

    <svelte:component this={pages[selected][0]}></svelte:component>

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