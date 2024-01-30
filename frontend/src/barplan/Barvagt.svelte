<script lang='ts'>
    import type { Vagt } from "./type";

    export let vagt : Vagt;
</script>

<tr>
    <td class="col-md-2">{vagt.date}</td>
    <td class="col-md-3">{vagt.responsible}</td>
    <td class="col-md-7">{vagt.bartendere}</td>
</tr>