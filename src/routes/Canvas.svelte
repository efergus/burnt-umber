<script lang="ts">
    import { centerplane, createProgram, createShader, plane } from "$lib/mesh";
    import vertsrc from "$lib/shaders/vert.glsl?raw"
    import fragsrc from "$lib/shaders/frag.glsl?raw"
    import { onMount } from "svelte";
    import init, { ColorView } from "$lib/rust";
    // import { new_canvas } from "wasm3d";
    export let r = 0;
    export let g = 0;
    export let b = 0;
    let canvas: HTMLCanvasElement;
    let program: WebGLProgram;
    let vaos: (WebGLVertexArrayObject | null)[] = [];
    // const resizeObserver = new ResizeObserver(onResize);
    // $: resizeObserver?.observe(canvas, {box: 'content-box'});

    // function onResize() {
    //     canvas.width = canvas.getBoundingClientRect().width;
    //     canvas.height = canvas.getBoundingClientRect().height;
    // }

    const initializePosition = (gl: WebGL2RenderingContext) => {
        let positionLoc = gl.getAttribLocation(program, "a_position")
        let positionBuf = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, positionBuf);
        let verts = centerplane();
        gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(verts), gl.STATIC_DRAW)
        let vao = gl.createVertexArray();
        vaos = [...vaos, vao];
        gl.bindVertexArray(vao);
        gl.enableVertexAttribArray(positionLoc);
        let size = 3;
        let type = gl.FLOAT;
        let normalize = false;
        let stride = 0;
        let offset = 0;
        gl.vertexAttribPointer(positionLoc, size, type, normalize, stride, offset)
    }

    const initializeColor = (gl: WebGL2RenderingContext) => {
        let positionLoc = gl.getAttribLocation(program, "a_color")
        let positionBuf = gl.createBuffer();
        gl.bindBuffer(gl.ARRAY_BUFFER, positionBuf);
        let verts = plane();
        gl.bufferData(gl.ARRAY_BUFFER, new Float32Array(verts), gl.STATIC_DRAW)
        let vao = gl.createVertexArray();
        vaos = [...vaos, vao];
        // gl.bindVertexArray(vao);
        gl.enableVertexAttribArray(positionLoc);
        let size = 3;
        let type = gl.FLOAT;
        let normalize = false;
        let stride = 0;
        let offset = 0;
        gl.vertexAttribPointer(positionLoc, size, type, normalize, stride, offset)
    }

    const initialize = (canvas: HTMLCanvasElement) => {
        const gl = canvas.getContext("webgl2");
        if(!gl) throw Error("Failed to get WebGL context")
        let vert = createShader(gl, gl.VERTEX_SHADER, vertsrc);
        let frag = createShader(gl, gl.FRAGMENT_SHADER, fragsrc);
        program = createProgram(gl, vert, frag);
        initializePosition(gl);
        initializeColor(gl);
    }

    const render = () => {
        const gl = canvas.getContext('webgl2')
        if(!gl) throw Error("Failed to get WebGL context")
        canvas.width = Math.round(canvas.clientWidth * window.devicePixelRatio)
        canvas.height = Math.round(canvas.clientHeight * window.devicePixelRatio)
        gl.viewport(0, 0, canvas.width, canvas.height);
        gl.clearColor(0, 0, 0, 0);
        gl.clear(gl.COLOR_BUFFER_BIT);
        gl.useProgram(program);
        console.log(vaos);
        for(let vao of vaos) {
            console.log(vao);
            gl.bindVertexArray(vao);
            let primitiveType = gl.TRIANGLES;
            let offset = 0;
            let count = 6;
            gl.drawArrays(primitiveType, offset, count);
        }
    }

    onMount(async ()=>{
        // initialize(canvas);
        // render();
        await init();
        const rect = canvas.getBoundingClientRect();
        const view = ColorView.new(canvas, rect.width, rect.height);
        console.log(view);
        view.render_loop();
        // new_canvas(canvas, rect.width, rect.height, (x: number, y: number, z: number)=>{
        //     console.log(x, y, z);
        //     r = x;
        //     g = y;
        //     b = z;
        // });
        // canvas.addEventListener("mousemove", pick);
    });
</script>

<!-- <div on:mousemove={pick}> -->
<canvas class="h-full w-full" bind:this={canvas}/>
<!-- </div> -->