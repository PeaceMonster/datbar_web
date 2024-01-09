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
            Nuværende Bartendere
        </li>

    </ul>
    
    <ul class="list-group tidligere">
        <li class="list-group-item list-group-item-warning">
            Tidligere Bartendere
        </li>
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