<script lang="ts">
    import { sx } from '$lib/classes';
    import { spring } from 'svelte/motion';
    import '../app.css';
    import RustCanvas from './RustCanvas.svelte';

    // greet("name");
    // console.log(new_canvas("string"));
    let r = 0;
    let g = 0;
    let b = 0;
    let hsv = '';
    let space = 'cylindrical';
    let color = spring([r, g, b]);
    let style: string;
    $: $color = [r, g, b];
    $: {
        style = sx({ bg: $color });
    }
    $: {
        console.log(space);
    }
</script>

<div class="flex flex-col justify-center">
    <div class="w-96 h-96">
        <RustCanvas bind:r bind:g bind:b />
    </div>
    <button
        on:click={() => {
            space = 'linear';
            console.log('AA', space);
        }}>BUTTON</button
    >
    <p {style}>rgb({r.toFixed(2)}, {g.toFixed(2)}, {b.toFixed(2)})</p>
</div>
