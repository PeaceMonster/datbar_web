<script lang='ts'>
    

    let last_page = false;
</script>

<h2>Barplan</h2>

{#if !last_page}
<a href="/" on:click|preventDefault={() => {}}>Vis tidligere</a>
{/if}

