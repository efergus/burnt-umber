<script lang="ts">
    import '../app.css';
    import Canvas from './Canvas.svelte';
    // import Big from './Big.svelte';
    import Center from './Center.svelte';
    import Colorpicker from './Colorpicker.svelte';
    // import { greet } from "wasm3d";

    // greet("name");
    // console.log(new_canvas("string"));
</script>

<div>
    <!-- Loading -->
    <Center>
        <!-- <Colorpicker /> -->
        <p class="w-96 h-96">
            <Canvas/>
        </p>
    </Center>
</div>

<style>
    div {
        position: absolute;
        width: 100vw;
        height: 100vh;
        /* background-color: blueviolet; */
    }
</style>
