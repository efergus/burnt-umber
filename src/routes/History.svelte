<script lang="ts">
    import { sx } from '$lib/classes';
    import type { Color } from '$lib/color';
    import { vec3 } from '$lib/geometry/vec';
    import ColorChip from './ColorChip.svelte';

    export let colors: Color[] = [];
    export let onClick: undefined | ((c: Color) => void) = undefined;

    function push(c: Color) {
        colors.push(c);
        colors = colors;
    }
    export const select = (c: Color, save = true) => {
        const l = colors.length;
        if (!l) {
            return push(c);
        }
        const current = colors[l - 1];
        if (save && current.near(c) && !colors[l - 2]?.near(c)) {
            return push(c);
        }
        colors[l - 1] = c;
    };
</script>

<div
    class="grid min-h-chip gap-1 bg-slate-100 overflow-x-auto"
    style={sx({
        columns: 'repeat( auto-fill, minmax(64px, 1fr))'
    })}
>
    {#each colors as color, index}
        <div>
            <ColorChip {color} selected={index === colors.length - 1} {onClick} />
        </div>
    {/each}
</div>