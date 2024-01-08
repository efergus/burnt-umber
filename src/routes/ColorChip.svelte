<script lang="ts">
    import { sx } from '$lib/classes';
    import { Color } from '$lib/color';
    import { vec3, type Vec3 } from '$lib/geometry/vec';

    export let color = new Color('srgb', vec3(1, 1, 1));
    export let onClick: undefined | ((c: Color) => void) = undefined;
</script>

<button
    class="w-full h-full min-h-[4rem] min-w-[4rem]"
    style={sx({
        bg: color.to_css()
    })}
    on:click={() => onClick?.(color)}
>
    <div class="absolute">
        <!-- {color.toString({ precision: 2 })} -->
    </div>
</button>
