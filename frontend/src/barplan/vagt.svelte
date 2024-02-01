<script lang='ts'>
    import type { Barvagt } from "../lib/types";
    import { get_short_month, expand_bartender_list } from "./script";

    export let vagt : Barvagt;
</script>

<tr>
    <td class="col-md-2">{get_short_month(vagt.date.getMonth())}. {vagt.date.getDate()}</td>
    <td class="col-md-3">{vagt.resposible.name}</td>
    <td class="col-md-7">{expand_bartender_list(vagt.bartenders)}</td>
</tr>