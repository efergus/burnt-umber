<script lang="ts">
    import * as THREE from 'three';
    import { frag, vert } from '$lib/shaders';
    import embed, {
        black_shader,
        cylindrical_shader,
        grey_shader,
        hsv_shader,
        rgb_shader,
        tDiffuse_shader,
        white_shader
    } from '$lib/shaders/embed';
    import { space, type Space } from '$lib/element/space';
    import { cameraController } from '$lib/element/controller';
    import { AXIS, Axis } from '$lib/element/axis';
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();
    export let color = [1, 1, 1];
    export let active: HTMLElement | undefined;
    export let slice = 1;
    let canvas: HTMLCanvasElement;
    let colorspace: Space;
    let saved_color = [...color];

    // export let thing_happened: (color: number[])=>void;

    const start = (canvas: HTMLCanvasElement) => {
        if (!canvas) return;
        colorspace = ColorSpace.new({ canvas, color, slice });
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
