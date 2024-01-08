<script lang="ts">
    import { sx } from '$lib/classes';
    import { Color } from '$lib/color';
    import { vec3, type Vec3 } from '$lib/geometry/vec';

    export let selected = false;
    export let color = new Color('srgb', vec3(1, 1, 1));
    export let onClick: undefined | ((c: Color) => void) = undefined;

    $: css = color.to_css();
</script>

<button
    class="w-full h-full min-h-chip min-w-chip border-2 hover:!border-white"
    style={sx({
        bg: css,
        border_color: selected ? 'black' : css
    })}
    on:click={() => onClick?.(color)}
>
    <div class="absolute">
        <!-- {color.toString({ precision: 2 })} -->
    </div>
</button>
