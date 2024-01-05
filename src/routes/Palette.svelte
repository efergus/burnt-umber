<script lang="ts">
    import { sx } from '$lib/classes';
    import type { Color } from '$lib/color';
    import { vec3 } from '$lib/geometry/vec';
    import ColorChip from './ColorChip.svelte';

    export let color: Color;
    export let onClick: undefined | ((c: Color) => void) = undefined;

    let palette_colors: Color[];
    $: palette_colors = new Array(6).fill(color).map((_, i) => {
        return color.add_norm(vec3(0.1 * i, -0.08 * i, 0));
    });
</script>

<div class="flex flex-col">
    {#each palette_colors as color}
        <div class="grow">
            <ColorChip {color} {onClick} />
        </div>
    {/each}
</div>
