<script lang="ts">
    import { sx } from '$lib/classes';
    import type Color from 'colorjs.io';
    import ColorChip from './ColorChip.svelte';

    export let color: Color;

    let palette_colors: Color[];
    $: palette_colors = new Array(6).fill(color).map((_, i) => {
        const new_color = color.clone();
        new_color.hsv.h += i * 20;
        return new_color;
    });
</script>

<div class="flex flex-col">
    {#each palette_colors as color}
        <div class="grow">
            <ColorChip {color} />
        </div>
    {/each}
</div>
