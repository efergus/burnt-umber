<script lang="ts">
    import { sx } from '$lib/classes';
    import type { Color } from '$lib/color';
    import { vec3 } from '$lib/geometry/vec';
    import ColorChip from './ColorChip.svelte';
    import TextPreview from './TextPreview.svelte';

    export let color: Color;
</script>

<div
    class="grid min-h-32 gap-4"
    style={sx({
        columns: 'repeat( auto-fill, minmax(16rem, 1fr))'
    })}
>
    <TextPreview {color} white={color.is_dark()} />
    <TextPreview {color} white={color.is_dark()} text />
</div>
