<script lang="ts">

    type Bartender = {
        name: string,
        username: string
    }

    let active : Bartender[] = [];
    let inactive : Bartender[] = [];

</script>


<h2>Bartendere</h2>

<div class="row">
    
    <ul class="list-group nuværende">
        <li class="list-group-item list-group-item-success">
            <strong>Nuværende Bartendere ({active.length})</strong>
        </li>
        {#each active as bar}
            <li class="list-group-item">
                {bar.name} (
                    <code>
                        {bar.username}
                    </code>
                )
            </li>
        {/each}
    </ul>
    
    <ul class="list-group tidligere">
        <li class="list-group-item list-group-item-warning">
            <strong>Tidligere Bartendere ({inactive.length})</strong>
        </li>
        {#each inactive as bar}
            <li class="list-group-item">
                {bar.name} (
                    <code>
                        {bar.username}
                    </code>
                )
            </li>
        {/each}
    </ul>
    
</div>

<style>
    .row {
        display: grid;
        grid-template-columns: 1fr 10px 1fr;
        grid-template-rows: 1fr;
        grid-template-areas: "nuværende . tidligere";
        width: 100%;
    }

    .nuværende {
        grid-area: nuværende;
    }

    .tidligere {
        grid-area: tidligere;
    }
</style>