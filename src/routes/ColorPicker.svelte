<script lang="ts">
    import {
        cylindrical,
        hsv,
    } from '$lib/shaders/embed';
    import { ColorSpace, space, type Space } from '$lib/element/space';
    import { createEventDispatcher } from 'svelte';
    import { vec3, type Vec3 } from '$lib/geometry/vec';

    const dispatch = createEventDispatcher();
    export let color = vec3(1, 1, 1);
    export let slice = 1;
    let canvas: HTMLCanvasElement;
    let colorspace: ColorSpace;

    // export let thing_happened: (color: number[])=>void;

    const start = (canvas: HTMLCanvasElement) => {
        if (!canvas) return;
        colorspace = ColorSpace.new({
            canvas,
            color,
            slice,
            space_embedding: cylindrical,
            color_embedding: hsv,

            onChange: (color) => {
                dispatch('change', color);
            }
        });
        render();
    };

    const render = () => {
        colorspace?.render();
        requestAnimationFrame(render)
    };

    export const set_color = (color: Vec3) => {
        colorspace?.set({ color });
    };

    $: start(canvas);
    $: set_color(color);
</script>

<div class="w-96 h-96">
    <canvas class="w-full h-full" bind:this={canvas} />
</div>
