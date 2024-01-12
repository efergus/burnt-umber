<script lang="ts">
    import { cartesian, cylindrical, hsv } from '$lib/shaders/embed';
    import { ColorSpace } from '$lib/element/space';
    import { createEventDispatcher } from 'svelte';
    import { vec3, type Vec3 } from '$lib/geometry/vec';
    import type { CursorSpec } from '$lib/element/cursor';
    import { Color } from '$lib/color';

    // const dispatch = createEventDispatcher<Vec3>();
    export let color = vec3(1, 1, 1);
    export let saved_color = vec3(1, 1, 1);
    export let slice = 1;
    export let cursors: CursorSpec[] = [];
    export let onClick: undefined | ((c: Color) => void) = undefined;
    export let slice_direction = 'horizontal';
    let canvas: HTMLCanvasElement;
    let colorspace: ColorSpace;

    const start = (canvas: HTMLCanvasElement) => {
        if (!canvas) return;
        colorspace = ColorSpace.new({
            canvas,
            color,
            slice,
            space_embedding: cartesian,
            color_embedding: hsv,

            slice_direction,

            onChange: ({ color: c, saved_color: s }) => {
                color = c;
                if (s) {
                    saved_color = s;
                    onClick?.(new Color('hsv', s));
                }
            }
        });
        render();
    };

    const render = () => {
        colorspace?.render(cursors);
        requestAnimationFrame(render);
    };

    export const set_color = (color: Vec3, saved_color: Vec3) => {
        colorspace?.set({ color, saved_color });
    };

    $: start(canvas);
    $: set_color(color, saved_color);
    $: colorspace?.set_slice(slice_direction);
</script>

<div class="w-96 h-96">
    <canvas class="w-full h-full" bind:this={canvas} />
</div>
