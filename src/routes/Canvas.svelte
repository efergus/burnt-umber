<script lang="ts">
    import vertsrc from '$lib/shaders/vert.glsl?raw';
    import fragsrc from '$lib/shaders/frag.glsl?raw';
    import { onMount } from 'svelte';
    import init, { ColorView } from '$lib/rust';
    // import { new_canvas } from "wasm3d";
    export let r = 0;
    export let g = 0;
    export let b = 0;
    let view: ColorView | undefined = undefined;
    let canvas: HTMLCanvasElement;
    let program: WebGLProgram;
    let vaos: (WebGLVertexArrayObject | null)[] = [];
    // const resizeObserver = new ResizeObserver(onResize);
    // $: resizeObserver?.observe(canvas, {box: 'content-box'});

    // function onResize() {
    //     canvas.width = canvas.getBoundingClientRect().width;
    //     canvas.height = canvas.getBoundingClientRect().height;
    // }
    // $: view?.set_space(space)

    const render = () => {
        const gl = canvas.getContext('webgl2');
        if (!gl) throw Error('Failed to get WebGL context');
        canvas.width = Math.round(canvas.clientWidth * window.devicePixelRatio);
        canvas.height = Math.round(canvas.clientHeight * window.devicePixelRatio);
        gl.viewport(0, 0, canvas.width, canvas.height);
        gl.clearColor(0, 0, 0, 0);
        gl.clear(gl.COLOR_BUFFER_BIT);
        gl.useProgram(program);
        console.log(vaos);
        for (let vao of vaos) {
            console.log(vao);
            gl.bindVertexArray(vao);
            let primitiveType = gl.TRIANGLES;
            let offset = 0;
            let count = 6;
            gl.drawArrays(primitiveType, offset, count);
        }
    };

    onMount(async () => {
        // initialize(canvas);
        // render();
        await init();
        const rect = canvas.getBoundingClientRect();
        view = ColorView.new(canvas, rect.width, rect.height, (x: number, y: number, z: number) => {
            r = x;
            g = y;
            b = z;
        });
        console.log(view);
        // view.set_space(space);
        view.render_loop();
        
    });
</script>

<!-- <div on:mousemove={pick}> -->
<canvas class="h-full w-full" bind:this={canvas} />
<!-- </div> -->
