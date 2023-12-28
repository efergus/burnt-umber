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
    import { cameraController } from '$lib/element/controller';
    import { AXIS, Axis } from '$lib/element/axis';
    import { cx } from '$lib/classes';
    import { vec3 } from '$lib/geometry/vec';
    import { createEventDispatcher } from 'svelte';

    const dispatch = createEventDispatcher();
    export let axis: AXIS;
    export let color = vec3(0.5, 1, 1);
    let canvas: HTMLCanvasElement;
    let axisElement: Axis;

    // const classnames = cx('border bg-gray-400', axis === AXIS.Y ? 'w-16 h-96' : 'w-96 h-16');

    const start = (canvas: HTMLCanvasElement) => {
        if (!canvas) return;

        axisElement = Axis.new(canvas, embed.hsv, axis, (c) => {
            color = c;
            // dispatch('change', c);
        });
        axisElement.on_input_change(new THREE.Vector3(...color));

        const start_time = Date.now();
        let last_time = Date.now();
        const animate = () => {
            const now = Date.now();
            const dt = now - last_time;
            last_time = now;
            requestAnimationFrame(animate);

            axisElement.render();
        };
        // canvas.oncontextmenu = (e) => {
        //     e.preventDefault();
        // };
        canvas.onmousemove = (e) => {
            axisElement.mouse_select(e);
        };
        animate();
    };

    export const set_color = (color: THREE.Vector3) => {
        axisElement?.set_color(color);
    };

    $: start(canvas);
    $: set_color(color);
</script>

<div class="w-full h-full">
    <canvas class="w-full h-full" bind:this={canvas} />
</div>
