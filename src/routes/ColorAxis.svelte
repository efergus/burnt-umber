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
    import { space } from '$lib/element/space';
    import { cameraController } from '$lib/element/controller';
    import { AXIS, Axis } from '$lib/element/axis';
    import { cx } from '$lib/classes';
    import { vec3 } from '$lib/geometry/vec';
    export let axis: AXIS;
    export let color = [0.5, 1, 1];
    let canvas: HTMLCanvasElement;
    let axisElement: Axis;

    // const classnames = cx('border bg-gray-400', axis === AXIS.Y ? 'w-16 h-96' : 'w-96 h-16');

    const start = (canvas: HTMLCanvasElement) => {
        if (!canvas) return;
        const rect = canvas.getBoundingClientRect();
        const renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true });
        renderer.setSize(rect.width, rect.height);
        renderer.autoClear = false;

        const scene = new THREE.Scene();
        const aspect = rect.width / rect.height;
        const width = aspect > 1 ? 1 : aspect;
        const height = aspect > 1 ? 1 / aspect : 1;
        console.log(width, height);
        const camera = new THREE.OrthographicCamera(
            -width / 2,
            width / 2,
            height / 2,
            -height / 2,
            0.1,
            100
        );
        camera.position.z = 1;
        camera.lookAt(0, 0, 0);

        const geometry = new THREE.BoxGeometry(1, 1, 1, 32, 8, 8);
        const boundingBox = new THREE.Box3().setFromObject(new THREE.Mesh(geometry));
        const embedMatrix = new THREE.Matrix4();
        embedMatrix.makeTranslation(boundingBox.min.multiplyScalar(-1));

        axisElement = Axis.new(embed.hsv, 1, axis);
        axisElement.on_input_change(vec3(...color));

        scene.add(...axisElement.meshes);

        const start_time = Date.now();
        let last_time = Date.now();
        const animate = () => {
            const now = Date.now();
            const dt = now - last_time;
            last_time = now;
            requestAnimationFrame(animate);

            renderer.setRenderTarget(null);
            renderer.clear();
            renderer.render(scene, camera);
        };
        const pick = (x: number, y: number) => {
            const px = x / rect.width;
            const py = y / rect.height;
            if (axis === AXIS.Y) {
                color[1] = py;
            } else {
                color[axis] = px;
            }
            axisElement.on_input_change(new THREE.Vector3(...color));
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
            // if (picked) {
            //     axisElement.on_input_change(picked.color);
            // }
            if (!e.buttons) {
                return;
            }
        };
        canvas.onwheel = (e) => {
            const dy = e.deltaY / 10;
        };
        animate();
    };

    $: start(canvas);
    $: axisElement?.on_input_change(vec3(...color));
</script>

<div class="w-full h-full">
    <canvas class="w-full h-full" bind:this={canvas} />
</div>
