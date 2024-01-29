<script lang='ts'>
    import type { Person } from "./types";
    
    export let member : Person;



</script>
<div style="display: flex; flex-direction: row; align-items: center;">
    <div class="board-image" style="background-image: url({member.imageUrl});"></div>
    
    <div style="margin-left: 1em;">
        <b>{member.post}</b>
        <br>
        <em>{member.title}</em>
        <br>
        {member.name} (<code>{member.username}</code>)
    </div>
</div>

<style>
    .board-image {
        height: 100px;
        width: 100px;
        background-size: contain;
        background-position: center;
        background-repeat: no-repeat;
        flex-shrink: 0;
    }
</style>