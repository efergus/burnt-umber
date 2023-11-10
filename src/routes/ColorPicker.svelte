
<script lang="ts">
    import * as THREE from 'three';
    import { frag, vert } from '$lib/shaders';
    import { cylindricalToCartesian, cylindrical_shader, hsv_shader, step_shader } from '$lib/shaders/embed';
    let canvas: HTMLCanvasElement;

    const start = (canvas: HTMLCanvasElement) => {
        if (!canvas) return;
        const rect = canvas.getBoundingClientRect();
        const scene = new THREE.Scene();
        const pickingScene = new THREE.Scene();
        const camera = new THREE.PerspectiveCamera(75, 1, 0.1, 1000);
        const camera_state = {
            theta: 0.5,
            phi: 0.5,
            radius: 4,
        }
        const renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true });
        renderer.setSize(rect.width, rect.height);
        // renderer.setClearColor(0xff0000, 0.8);

        const geometry = new THREE.BoxGeometry(1, 1, 1, 32, 8, 8);
        const cursorGeometry = new THREE.SphereGeometry(0.06, 8, 8);
        const pickGeometry = geometry.clone();
        // geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));
        // geometry.setAttribute('color', new THREE.BufferAttribute(color, 3));
        // const material = new THREE.MeshBasicMaterial();
        // const material = new THREE.Material();
        const boundingBox = new THREE.Box3().setFromObject(new THREE.Mesh(geometry));
        const embedMatrix = new THREE.Matrix4();
        embedMatrix.makeTranslation(boundingBox.min.multiplyScalar(-1));
        // embedMatrix.scale(new THREE.Vector3(0.5, 0.5, 0.5));
        const material = new THREE.ShaderMaterial({
            vertexShader: vert(cylindrical_shader),
            fragmentShader: frag(hsv_shader),
            uniforms: {
                embedMatrix: { value: embedMatrix },
            }
        })
        const cursorMaterial = new THREE.ShaderMaterial({
            vertexShader: vert(),
            fragmentShader: frag(),
            uniforms: {
                embedMatrix: { value: new THREE.Matrix4() },
            }
        })
        const pickMaterial = new THREE.ShaderMaterial({
            vertexShader: vert(cylindrical_shader),
            fragmentShader: frag(),
            uniforms: {
                embedMatrix: { value: embedMatrix },
            }
        })
        const cube = new THREE.Mesh(geometry, material);
        const cursor = new THREE.Mesh(cursorGeometry, cursorMaterial);
        const pickCube = new THREE.Mesh(pickGeometry, pickMaterial);
        const pickTarget = new THREE.WebGLRenderTarget(rect.width, rect.height, {
            format: THREE.RGBAFormat,
            type: THREE.FloatType,
        });
        pickTarget.texture.generateMipmaps = false;
        scene.add(cube);
        scene.add(cursor);
        pickingScene.add(pickCube);

        camera.position.z = 2;

        const start_time = Date.now();
        const animate = () => {
            requestAnimationFrame(animate);
            const time = (Date.now() - start_time) * 0.001;
            camera.position.x = camera_state.radius * Math.cos(camera_state.theta) * Math.cos(camera_state.phi);
            camera.position.y = camera_state.radius * Math.sin(camera_state.theta);
            camera.position.z = camera_state.radius * Math.cos(camera_state.theta) * Math.sin(camera_state.phi);
            camera.lookAt(cube.position);
            renderer.render(scene, camera);
        }
        const pick = (x: number, y: number) => {
            // console.log(x, y);
            renderer.setRenderTarget(pickTarget);
            renderer.render(pickingScene, camera);
            const pixelBuffer = new Float32Array(4);
            const gl = renderer.getContext();
            if(!gl) {
                console.log(
                    "No context!"
                )
                return;
            }
            // console.log(gl);
            // console.log(gl.getRenderbufferParameter(gl.RENDERBUFFER, gl.RENDERBUFFER_INTERNAL_FORMAT));
            gl.readPixels(x, y, 1, 1, gl.RGBA, gl.FLOAT, pixelBuffer);
            if (pixelBuffer[3] === 0) {
                renderer.setRenderTarget(null);
                return;
            }
            
            const colorPosition = new THREE.Vector3(pixelBuffer[0], pixelBuffer[1], pixelBuffer[2]);
            const cursorPosition = cylindricalToCartesian(colorPosition);
            // const cursorEmbed = new THREE.Matrix4();
            // cursorEmbed.makeTranslation(cursorPosition);

            cursor.position.copy(cursorPosition);
            cursor.material.uniformsNeedUpdate = true;
            renderer.setRenderTarget(null);
        }
        canvas.onmousemove = (e) => {
            const rect = canvas.getBoundingClientRect();
            pick(e.clientX - rect.left, rect.bottom - e.clientY);
            if (!e.buttons) {
                return;
            }
            // console.log(e.buttons);
            const dx = e.movementX / rect.width;
            const dy = e.movementY / rect.width;
            camera_state.theta += dy;
            camera_state.phi += dx;
            console.log(camera_state);

            // camera.position.x += dx;
            // camera.position.y -= dy;
            // cube.rotation.x += dy * 2;
            // cube.rotation.z += dx * 2;
        }
        canvas.onwheel = (e) => {
            camera_state.radius *= (1.0 + e.deltaY / 1000);
            console.log(camera_state);
        }
        animate();
    }

    $: start(canvas);
</script>

<div class='w-96 h-96 border bg-gray-400'>
    <canvas class='w-full h-full' bind:this={canvas}/>
</div>