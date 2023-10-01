<script lang="ts">
    import { sx } from '$lib/classes';

    export let x = 0;
    export let y = 0;
    export let color = [0, 0, 0];
    export let axes: { x?: boolean; y?: boolean };
    $: style = sx({
        pos: 'absolute',
        w: axes.y && !(axes.x && axes.y) ? '100%' : 0,
        h: axes.x && !(axes.x && axes.y) ? '100%' : 0,
        x,
        bottom: y
    });
</script>

<div class="flex justify-center items-center w-0 absolute overflow-visible" {style}>
    <div style={sx({ p: 2, w: '100%', h: '100%', bg: color })} />
</div>
