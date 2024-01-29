<script lang='ts'>
    
    import EventComponent from "./EventComponent.svelte";


    let events = [];

</script>

<h2>Bartender events</h2>

<p>
    <a href="?show_all">Vis også tidligere</a>    
</p>
<p>
    <a href="/events.ics">Tilføj til kalender</a>
</p>

<EventComponent/>