<script lang='ts'>
    import type { Barvagt } from "../lib/types";
    import { get_barplan } from "./script";
    import Vagt from "./vagt.svelte";
    let last_page = false;

    let vagter : Barvagt[] = [];

    get_barplan(new Date).then((r) => {
        vagter = r;
        console.log(r);
    })
</script>

<h2>Barplan</h2>

{#if !last_page}
<a href="/" on:click|preventDefault={() => {}}>Vis tidligere</a>
{/if}

<table class="table table-striped tablesorter">
    <thead>
        <tr>
            <th>Dato</th>
            <th>Ansvarlig</th>
            <th>Bartendere</th>
        </tr>
    </thead>
    <tbody>
        {#each vagter as vagt}
            <Vagt vagt={vagt}/>
        {/each}
    </tbody>
</table>
