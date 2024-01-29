<script lang='ts'>
    import type { Bestyrelse } from "./types";
    import BestyrelsesElement from "./BestyrelsesElement.svelte";

    let test : Bestyrelse = {
        periode: "Test",
        members: [
            {
                post: "Test",
                title: "Testing",
                name: "Test Person",
                username: "Test",
                imageUrl: "Not Found",
            },
            {
                post: "Test",
                title: "Testing",
                name: "Test Person",
                username: "Test",
                imageUrl: "Not Found",
            },
            {
                post: "Test",
                title: "Testing",
                name: "Test Person",
                username: "Test",
                imageUrl: "Not Found",
            },
        ]
    }
</script>

<BestyrelsesElement bestyrelse={test} collapse={true}/>