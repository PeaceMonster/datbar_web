<script lang='ts'>
    import type { Bestyrelse } from "./types";
    import Member from "./Member.svelte";

    export let bestyrelse : Bestyrelse;

    export let collapse = false;

    function toggleCollapse() {
        collapse = !collapse;
    }

</script>


<div class="panel-group">
    <div class="panel panel-default">
        <div class="panel-heading __web-inspector-hide-shortcut__">
			<h4 class="panel-title">
				<a data-toggle="collapse" data-parent="#accordion" href="#collapse1" on:click|preventDefault={toggleCollapse}>
					Bestyrelsen {bestyrelse.periode}
				</a>
				
			</h4>
		</div>
        <div id="collapse1" class="panel-collapse collapse" class:in={collapse}>
			<div class="panel-body">
				<ul class="list-group ">
                    {#each bestyrelse.members as member}
                        <li class="list-group-item col-md-6">
                            <Member member={member}/>
                        </li>
                    {/each}
				</ul>
			</div>
		</div>
    </div>
</div>