<script>
    import { sx } from '$lib/classes';

    export let color = [0.5, 1, 1];
</script>

<div
    class="w-full h-full"
    style={sx({
        bg: `hsl(${color[0] * 360}, ${color[2] * 100}%, ${color[1] * 100}%)`
    })}
/>
