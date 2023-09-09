<script lang="ts">
    import { sx } from '$lib/classes';
    import '../app.css';
    import Canvas from './Canvas.svelte';
    // import Big from './Big.svelte';
    import Center from './Center.svelte';
    import Colorpicker from './Colorpicker.svelte';
    // import { greet } from "wasm3d";

    // greet("name");
    // console.log(new_canvas("string"));
    let r = 0;
    let g = 0;
    let b = 0;
    let style: string;
    $: {
        style = sx({bg: [r/255, g/255, b/255]});
        console.log(style);
    }
</script>

<div>
    <!-- Loading -->
    <Center>
        <!-- <Colorpicker /> -->
        <div>
            <p class="w-96 h-96">
                <Canvas bind:r={r} bind:g={g} bind:b={b}/>
            </p>
            <div {style}>
                AAAA
            </div>
        </div>
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
