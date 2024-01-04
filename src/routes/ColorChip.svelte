<script>
    import { sx } from '$lib/classes';
    import { vec3 } from '$lib/geometry/vec';
    import Color from 'colorjs.io';

    export let color = new Color('srgb', [1, 1, 1]);
</script>

<div
    class="w-full h-full min-h-[4rem] min-w-[4rem]"
    style={sx({
        bg: color.display().toString()
    })}
>
    <div class="absolute">
        {color.toString({ precision: 2 })}
    </div>
</div>
