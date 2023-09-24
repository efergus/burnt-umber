<script lang="ts">
    import { sx } from '$lib/classes';
    import { spring } from 'svelte/motion';
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
    let space = "cylindrical";
    let color = spring([0.0, 0.0, 0.0])
    let style: string;
    $: $color = [r, g, b]
    $: {
        style = sx({bg: $color});
        // console.log(style);
    }
    $: {
        console.log(space)
    }
</script>

<div id="main">
    <!-- Loading -->
    <Center>
        <!-- <Colorpicker /> -->
        <div class="w-96 h-96">
            <Canvas bind:r={r} bind:g={g} bind:b={b} bind:space={space}/>
        </div>
        <button on:click={()=>{
            space = "linear"
            console.log("AA", space)
        }}>BUTTON</button>
    </Center>
</div>

<style>
    div#main {
        position: absolute;
        width: 100vw;
        height: 100vh;
        /* background-color: blueviolet; */
    }
</style>
