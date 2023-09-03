<script lang="ts">
    import { sx } from '$lib/classes';
    import { spring } from 'svelte/motion';
    import ColorAxes from './ColorAxes.svelte';

    export let r = 0;
    export let g = 0;
    export let b = 0;
    export let a = 1;
    let saved = {r, g, b, a};
    let coords = spring({ r, g, b, a }, { stiffness: 0.3, damping: 1 });
    $: $coords = {r, g, b, a}

    const style = sx({
        display: 'grid',
        columns: '4rem 1fr 4rem',
        rows: '4rem 1fr 4rem',
        gap: '0.5rem',
        w: 400,
        h: 400
    });
</script>

<div {style}>
    <!-- row 1 -->
    <div style={sx({ bg: [$coords.r, $coords.g, $coords.b, $coords.a], w: '100%', h: '100%' })} />
    <ColorAxes bind:x={g} bind:savedX={saved.g} axes={{ x: true }} />
    <div />
    <!-- row 2 -->
    <ColorAxes bind:y={r} bind:savedY={saved.r} axes={{ y: true }} />
    <ColorAxes bind:x={g} bind:y={r} bind:savedX={saved.g} bind:savedY={saved.r} />
    <ColorAxes bind:y={b} bind:savedY={saved.b} axes={{ y: true }} />
    <!-- row 3 -->
    <div />
    <ColorAxes bind:x={a} axes={{ x: true }} />
    <div />
</div>
