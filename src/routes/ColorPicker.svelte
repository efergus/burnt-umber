
<script lang="ts">
    import * as THREE from 'three';
    import { frag, vert } from '$lib/shaders';
    import embed from '$lib/shaders/embed';
    import { space } from '$lib/element/space';
    import { cameraController } from '$lib/element/controller';
    let canvas: HTMLCanvasElement;

    const unit = {
        x: new THREE.Vector3(1, 0, 0),
        y: new THREE.Vector3(0, 1, 0),
        z: new THREE.Vector3(0, 0, 1),
    }

    const start = (canvas: HTMLCanvasElement) => {
        if (!canvas) return;
        const rect = canvas.getBoundingClientRect();
        const scene = new THREE.Scene();
        const pickingScene = new THREE.Scene();
        const camera = new THREE.PerspectiveCamera(75, 1, 0.01, 100);
        const controller = cameraController(camera);
        const renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true });
        renderer.setSize(rect.width, rect.height);
        // renderer.setClearColor(0xff0000, 0.8);

        const geometry = new THREE.BoxGeometry(1, 1, 1, 32, 8, 8);
        const cursorGeometry = new THREE.SphereGeometry(0.06, 8, 8);
        // geometry.setAttribute('position', new THREE.BufferAttribute(vertices, 3));
        // geometry.setAttribute('color', new THREE.BufferAttribute(color, 3));
        // const material = new THREE.MeshBasicMaterial();
        // const material = new THREE.Material();
        const boundingBox = new THREE.Box3().setFromObject(new THREE.Mesh(geometry));
        const embedMatrix = new THREE.Matrix4();
        embedMatrix.makeTranslation(boundingBox.min.multiplyScalar(-1));
        
        const colorspace = space(embed.cylindrical, embed.hsv, 1);
        const pickTarget = new THREE.WebGLRenderTarget(rect.width, rect.height, {
            format: THREE.RGBAFormat,
            type: THREE.FloatType,
        });
        pickTarget.texture.generateMipmaps = false;
        scene.add(...colorspace.meshes);
        pickingScene.add(...(colorspace.pick_meshes ?? []));

        const state = {
            selected: new THREE.Vector3(0, 0, 0),
            space: new THREE.Vector3(0, 0, 0),
        };

        const start_time = Date.now();
        let last_time = Date.now();
        const animate = () => {
            const now = Date.now();
            const dt = now - last_time;
            last_time = now;
            requestAnimationFrame(animate);
            // const position = camera.position.clone();
            // const position = new THREE.Vector3(
            //     camera_state.radius * Math.cos(camera_state.theta) * Math.cos(camera_state.phi),
            //     camera_state.radius * Math.sin(camera_state.theta),
            //     camera_state.radius * Math.cos(camera_state.theta) * Math.sin(camera_state.phi),
            // );
            // position.add(camera_state.lookAt);
            // const new_up = unit.y.clone();
            // new_up.sub(camera_state.up);
            // camera_state.up.addScaledVector(new_up, dt/100);
            // camera_state.up.normalize();
            // cursor.scale.setScalar(camera_state.radius / 4);
            // camera.up.copy(camera_state.up);
            // camera.lookAt(camera_state.lookAt);
            renderer.render(scene, camera);
        }
        const pick = (x: number, y: number) => {
            // console.log(x, y);
            renderer.setRenderTarget(pickTarget);
            renderer.render(pickingScene, camera);
            const pixelBuffer = new Float32Array(4);
            const gl = renderer.getContext();
            if(!gl) {
                console.error(
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
            // const spacePosition = cylindricalToCartesian(colorPosition);
            // console.log(pixelBuffer[3]);
            // const cursorEmbed = new THREE.Matrix4();
            // cursorEmbed.makeTranslation(cursorPosition);

            // console.log("Color", ...colorPosition)
            // state.selected.copy(colorPosition);
            // cursor.position.copy(cursorPosition);
            // state.space.copy(cursorPosition);
            cursor.material.uniformsNeedUpdate = true;
            renderer.setRenderTarget(null);
            return {
                color: colorPosition,
                // space: spacePosition,
            };
        }
        canvas.oncontextmenu = (e) => {
            e.preventDefault();
            const rect = canvas.getBoundingClientRect();
            const picked = pick(e.clientX - rect.left, rect.bottom - e.clientY);
            console.log(e.button);
            // if (picked && e.button === 2) {
            //     const diff = picked.space.clone();
            //     diff.sub(camera_state.lookAt);
            //     camera_state.lookAt.copy(picked.space);
            //     camera.position.add(diff);
            // }
        }
        canvas.onmousemove = (e) => {
            const rect = canvas.getBoundingClientRect();
            const picked = pick(e.clientX - rect.left, rect.bottom - e.clientY);
            if (picked) {
                // console.log(...picked.space)
                // state.selected.copy(picked.color);
                // state.space.copy(picked.space);
                // cursor.position.copy(state.space);
                colorspace.on_input_change(picked.color);
            }
            if (!e.buttons) {
                return;
            }
            controller.on_move(new THREE.Vector3(e.movementX, e.movementY, 0.0));
            // const radius = Math.sqrt(camera.position.x ** 2 + camera.position.z ** 2);
            // let theta = Math.atan2(camera.position.y, radius);
            // let phi = Math.atan2(camera.position.z, camera.position.x);
            // const pos = camera.position.clone();
            // pos.sub(camera_state.lookAt);
            // const unit_x = pos.clone();
            // const unit_y = pos.clone();
            // unit_x.cross(camera_state.up);
            // unit_y.cross(unit_x);
            // unit_x.normalize();
            // unit_y.normalize();
            // // console.log(e.buttons);
            // const xz_radius = Math.sqrt(pos.x ** 2 + pos.z ** 2);
            // const dx = e.movementX / rect.width * xz_radius * Math.PI;
            // const dy = - e.movementY / rect.width * camera_state.radius * Math.PI;
            // pos.addScaledVector(unit_x, dx);
            // pos.addScaledVector(unit_y, dy);
            // pos.multiplyScalar(camera_state.radius / pos.length());
            // pos.add(camera_state.lookAt);
            // camera.position.copy(pos);
            // pos.copy(camera.position);
            // pos.sub(camera_state.lookAt);
            // const up = unit_x.clone();
            // up.cross(pos).normalize();
            // if(up.dot(unit.y) > 0.2) {

            // }
            // camera_state.up.copy(up);
            // console.log(camera_state);

            // camera.position.x += dx;
            // camera.position.y -= dy;
            // cube.rotation.x += dy * 2;
            // cube.rotation.z += dx * 2;
        }
        canvas.onwheel = (e) => {
            const dy = e.deltaY / 10;
            // camera_state.radius += dy / 10;
            // camera_state.radius = Math.min(Math.max(camera_state.radius, 0.1), 4);

            // const diff = camera.position.clone();
            // diff.sub(state.space);
            // diff.multiplyScalar(1-Math.exp(-camera_state.radius));
            // camera.position.addScaledVector(diff, dy/100);
            // const pos = camera.position.clone();
            // pos.sub(camera_state.lookAt);
            // camera_state.radius = pos.length();
            controller.on_move(new THREE.Vector3(0, 0, dy));
        }
        animate();
    }

    $: start(canvas);
</script>

<div class='w-96 h-96 border bg-gray-400'>
    <canvas class='w-full h-full' bind:this={canvas}/>
</div>