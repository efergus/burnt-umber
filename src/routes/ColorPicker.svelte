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
    let canvas: HTMLCanvasElement;
    let colorspace: Space;

    // export let thing_happened: (color: number[])=>void;

    const start = (canvas: HTMLCanvasElement) => {
        if (!canvas) return;
        const rect = canvas.getBoundingClientRect();
        const scene = new THREE.Scene();
        const orthoScene = new THREE.Scene();
        const pickingScene = new THREE.Scene();
        const orthoPickingScene = new THREE.Scene();
        const camera = new THREE.PerspectiveCamera(75, 1, 0.01, 100);
        const controller = cameraController(camera);
        const orthoCamera = new THREE.OrthographicCamera(-1, 1, 1, -1, 0.1, 100);
        orthoCamera.position.z = 1;
        orthoCamera.lookAt(0, 0, 0);
        const renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true });
        // renderer.setClearColor(0x000000, 0);
        renderer.setSize(rect.width, rect.height);
        renderer.setPixelRatio(2);
        renderer.autoClear = false;

        const pickTarget = new THREE.WebGLRenderTarget(rect.width, rect.height, {
            format: THREE.RGBAFormat,
            type: THREE.FloatType
        });
        pickTarget.texture.generateMipmaps = false;
        const orthoTarget = new THREE.WebGLRenderTarget(rect.width, rect.height, {
            format: THREE.RGBAFormat
        });
        orthoTarget.texture.generateMipmaps = false;

        colorspace = space(embed.cylindrical, embed.hsv, 1);

        scene.add(...colorspace.meshes);
        pickingScene.add(...colorspace.pick_meshes);

        const start_time = Date.now();
        let last_time = Date.now();
        const animate = () => {
            const now = Date.now();
            const dt = now - last_time;
            last_time = now;
            requestAnimationFrame(animate);

            renderer.setRenderTarget(null);
            renderer.clear();
            renderer.render(orthoScene, orthoCamera);

            renderer.render(scene, camera);
        };
        const pick = (x: number, y: number) => {
            renderer.setRenderTarget(pickTarget);
            renderer.clear();
            renderer.render(pickingScene, camera);
            renderer.render(orthoPickingScene, orthoCamera);
            const pixelBuffer = new Float32Array(4);
            const gl = renderer.getContext();
            if (!gl) {
                console.error('No context!');
                return;
            }
            gl.readPixels(x, y, 1, 1, gl.RGBA, gl.FLOAT, pixelBuffer);
            if (pixelBuffer[3] === 0) {
                renderer.setRenderTarget(null);
                return;
            }

            color = [pixelBuffer[0], pixelBuffer[1], pixelBuffer[2]];
            const colorPosition = new THREE.Vector3(...color);
            renderer.setRenderTarget(null);
            return {
                color: colorPosition
                // space: spacePosition,
            };
        };
        canvas.oncontextmenu = (e) => {
            e.preventDefault();
            const rect = canvas.getBoundingClientRect();
            const picked = pick(e.clientX - rect.left, rect.bottom - e.clientY);
            console.log(e.button);
        };
        canvas.onmousemove = (e) => {
            const rect = canvas.getBoundingClientRect();
            const picked = pick(e.clientX - rect.left, rect.bottom - e.clientY);
            if (picked) {
                colorspace.on_input_change(picked.color, true);
                // thing_happened(color);
                // dispatch('change', color);
                active = canvas;
            }
            if (!e.buttons) {
                return;
            }
            controller.on_move(new THREE.Vector3(e.movementX, e.movementY, 0.0));
        };
        canvas.addEventListener('wheel', (e) => {
            const dy = e.deltaY / 10;
            controller.on_move(new THREE.Vector3(0, 0, dy));
        }, {
            passive: false,
        });
        animate();
    };

    export const set_color = (c: number[]) => {
        colorspace?.on_input_change(new THREE.Vector3(...c), active === canvas);
        color = c.slice();
    };

    $: start(canvas);
    $: set_color(color);
</script>

<div class="w-96 h-96">
    <canvas class="w-full h-full" bind:this={canvas} />
</div>
