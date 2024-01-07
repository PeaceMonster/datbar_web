<script lang='ts'>
    import Forside from "./forside/Forside.svelte";
    import Barplan from "./barplan/Barplan.svelte";

    let pages = [
        Forside, 
        Barplan,
    ];
    let selected : any = 0;
</script>

<main>
    <div class="header">
        <h1>
            <img src="logo.png" alt="" >
            Fredagscafeen
        </h1>
        <div class="flexy">
            <a href="/" on:click|preventDefault={() => selected = 0}>Forside</a>
            <a href="/" on:click|preventDefault={() => selected = 1M!}>Barplan</a>
        </div>
    </div>

    <svelte:component this={pages[selected]}></svelte:component>

</main>


<style>


    h1 {
        color: midnightblue;
        font-family: serif;
        display: inline-block;
    }
    .flexy {
        display: flex;
        width: 100%;
    }

    .header a {
        text-decoration: none;
        font-size: larger;
        display: inline-block;
        padding: 10px 20px;
    }

    img {
        height: 50px;
        display: inline-block;
    }
</style>