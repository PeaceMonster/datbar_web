<script lang='ts'>
    import type { Vagt } from "./type";
    import Barvagt from "./Barvagt.svelte";
    let last_page = false;

    let vagter : Vagt[] = [
        {
            date: "Feb 02",
            responsible: "VH",
            bartendere: "Erik Funder, Simon Dohn"
        }
    ];
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
            <Barvagt vagt={vagt}/>
        {/each}
    </tbody>
</table>
