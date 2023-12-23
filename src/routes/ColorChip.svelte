<script>
    import { sx } from '$lib/classes';
    import { vec3 } from '$lib/geometry/vec';

    export let color = vec3(0.5, 1, 1);
</script>

<div
    class="w-full h-full"
    style={sx({
        bg: `hsl(${color.x * 360}, ${color.z * 100}%, ${color.y * 100}%)`
    })}
/>
