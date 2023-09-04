<script lang="ts">
    import '../app.css';
    // import Big from './Big.svelte';
    import Center from './Center.svelte';
    import Colorpicker from './Colorpicker.svelte';
    // import { greet } from "wasm3d";
    const rust_init = import("$lib/rust/pkg/rust.js").then(({default: init})=>init());

    // greet("name");
    // console.log(new_canvas("string"));
</script>

<div>
    {#await rust_init}
    Loading
    {:then}
    <Center>
        <Colorpicker />
    </Center>
    {/await}
</div>

<style>
    div {
        position: absolute;
        width: 100vw;
        height: 100vh;
        /* background-color: blueviolet; */
    }
</style>
