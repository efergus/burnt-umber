import { near, vec3, type Mat4, type Vec3, type Vec2, vec2, near2, mat4, wrapAxis } from '$lib/geometry/vec';
import { definitions, frag, vert } from '$lib/shaders';
import {
    pick_shader,
    type Embedding,
    embed_shader,
    type CPUEmbedding,
} from '$lib/shaders/embed';
import * as THREE from 'three';
import { cameraController, type CameraController } from './controller';
import type { ColorElement } from '.';
import { AXIS, type ColorState } from './axis';
import { Cursor, setCursors, type CursorSpec } from './cursor';
import { spring, type Spring } from '$lib/motion/spring';

export interface Space extends ColorElement {
    space_embedding: Embedding;
    color_embedding: Embedding;
    input_pos: THREE.Vector3;
    slice: number;
}

class ColorSpaceCube {
    mesh: THREE.Mesh<THREE.PlaneGeometry, THREE.ShaderMaterial>;
    pick_mesh: THREE.Mesh<THREE.PlaneGeometry, THREE.ShaderMaterial>;
    space_embedding: Embedding;
    color_embedding: Embedding;
    base_embedding_matrix: Mat4;

    constructor(
        scene: THREE.Scene,
        pickScene: THREE.Scene,
        space_embedding: Embedding,
        color_embedding: Embedding
    ) {
        const geometry = new THREE.BoxGeometry(1, 1, 1, 64, 8, 8);
        const boundingBox = new THREE.Box3().setFromObject(new THREE.Mesh(geometry));
        const embedMatrix = new THREE.Matrix4();
        embedMatrix.makeTranslation(boundingBox.min.multiplyScalar(-1));

        const material = new THREE.ShaderMaterial({
            vertexShader: vert(embed_shader, space_embedding.shader),
            fragmentShader: definitions('USE_CLIP_PLANE') + frag(color_embedding.shader),
            uniforms: {
                clipPlane: { value: new THREE.Vector4(0, 0, 1, 1.1) },
                embedMatrix: { value: embedMatrix }
            }
        });
        const pick_material = new THREE.ShaderMaterial({
            vertexShader: vert(embed_shader, space_embedding.shader),
            fragmentShader: definitions('USE_CLIP_PLANE') + frag(pick_shader),
            uniforms: {
                clipPlane: { value: new THREE.Vector4(0, 0, 1, 1.1) },
                embedMatrix: { value: embedMatrix },
                tag: { value: 1 }
            }
        });

        this.mesh = new THREE.Mesh(geometry, material);
        this.pick_mesh = new THREE.Mesh(geometry.clone(), pick_material);

        scene.add(this.mesh);
        pickScene.add(this.pick_mesh);

        this.space_embedding = space_embedding;
        this.color_embedding = color_embedding;
        this.base_embedding_matrix = embedMatrix;
    }

    set(vertical_top: number, horizontal_start = 0, horizontal_size = 1) {
        const scale = mat4().makeScale(horizontal_size, vertical_top, 1);
        const trans = mat4().makeTranslation(vec3(horizontal_start, 0, 0))
        const embedMatrix = scale.multiply(trans).multiply(this.base_embedding_matrix)
        this.mesh.material.uniforms.embedMatrix.value = embedMatrix;
        this.pick_mesh.material.uniforms.embedMatrix.value = embedMatrix;
    }
}

type WithoutMethods<T> = {
    // eslint-disable-next-line @typescript-eslint/ban-types
    [K in keyof T as T[K] extends Function ? never : K]: T[K];
};

export interface ColorSpaceParams {
    canvas: HTMLCanvasElement;
    color: Vec3;
    space_embedding: CPUEmbedding;
    color_embedding: Embedding;
    slice: number;
    slice_direction: string;

    onChange?: (state: ColorState) => void;
}

export class ColorSpace {
    canvas: HTMLCanvasElement;
    color: Vec3;
    saved_color: Vec3;
    renderer: THREE.WebGLRenderer;
    screenScene: THREE.Scene;
    screenCamera: THREE.OrthographicCamera;
    cursorScene: THREE.Scene;
    colorScene: THREE.Scene;
    camera: THREE.PerspectiveCamera;
    cameraController: CameraController;
    pickScene: THREE.Scene;
    pickTarget: THREE.WebGLRenderTarget;
    cursorTarget: THREE.WebGLRenderTarget;
    colorTarget: THREE.WebGLRenderTarget;

    cube: ColorSpaceCube;
    cursors: Cursor[];

    select_position?: Vec2;
    slice_direction: string;

    spring: Spring;

    onChange?: (state: ColorState) => void;

    constructor({
        color,
        saved_color,
        canvas,
        renderer,
        screenScene,
        camera,
        cameraController,
        pickScene,
        pickTarget,
        cube,
        cursors,
        slice_direction,
        spring,
        onChange,
        cursorScene,
        cursorTarget,
        colorTarget,
        colorScene,
        screenCamera,
    }: WithoutMethods<ColorSpace>) {
        this.canvas = canvas;
        this.color = color;
        this.saved_color = saved_color;
        this.renderer = renderer;
        this.screenScene = screenScene;
        this.screenCamera = screenCamera;
        this.camera = camera;
        this.cameraController = cameraController;
        this.pickScene = pickScene;
        this.pickTarget = pickTarget;

        this.cube = cube;
        this.cursors = cursors;

        this.slice_direction = slice_direction;
        this.spring = spring;

        this.cursorScene = cursorScene;
        this.cursorTarget = cursorTarget;
        this.colorScene = colorScene;
        this.colorTarget = colorTarget;

        this.onChange = onChange;

        canvas.addEventListener('mousemove', (e) => {
            this.mouse_select(e);
        });
        canvas.addEventListener('mousedown', (e) => {
            this.mouse_select(e);
        });
        canvas.addEventListener('mouseup', (e) => {
            this.mouse_select(e);
        });
        canvas.addEventListener('mouseleave', (e) => {
            this.mouse_select(e);
        })
        canvas.addEventListener(
            'wheel',
            (e) => {
                e.preventDefault();
                const dy = e.deltaY / 10;
                cameraController.on_move(new THREE.Vector3(0, 0, dy));
            },
            {
                passive: false
            }
        );
    }
    static new(params: ColorSpaceParams) {
        const rect = params.canvas.getBoundingClientRect();
        const renderer = new THREE.WebGLRenderer({
            canvas: params.canvas,
            antialias: true,
            alpha: true
        });
        renderer.setSize(rect.width, rect.height);
        renderer.setPixelRatio(window.devicePixelRatio);
        renderer.autoClear = false;

        const camera = new THREE.PerspectiveCamera(40, 1, 0.01, 100);
        const orthoCamera = new THREE.OrthographicCamera(-1, 1, 1, -1, 0.1, 100);
        orthoCamera.position.z = 1;
        orthoCamera.lookAt(0, 0, 0);

        // TODO: do 1x1 and scissor?
        const pickTarget = new THREE.WebGLRenderTarget(rect.width, rect.height, {
            format: THREE.RGBAFormat,
            type: THREE.FloatType,
            generateMipmaps: false,
        });

        const colorTarget = new THREE.WebGLRenderTarget(rect.width * 2, rect.height * 2);
        const cursorTarget = new THREE.WebGLRenderTarget(rect.width * 2, rect.height * 2, {
            format: THREE.RGBAFormat,
            generateMipmaps: false,
        });

        const cursor_plane_geometry = new THREE.PlaneGeometry();
        const cursor_plane_material = new THREE.MeshBasicMaterial({ color: 0xffffff, map: cursorTarget.texture, transparent: true })
        const cursor_plane = new THREE.Mesh(cursor_plane_geometry, cursor_plane_material);
        cursor_plane.position.z = 0.1;
        cursor_plane.scale.copy(vec3(2, 2, 2))

        const color_plane_geometry = new THREE.PlaneGeometry();
        const color_plane_material = new THREE.MeshBasicMaterial({ color: 0xffffff, map: colorTarget.texture, transparent: true })
        const color_plane = new THREE.Mesh(color_plane_geometry, color_plane_material);
        color_plane.scale.copy(vec3(2, 2, 2))

        const screenScene = new THREE.Scene();
        const cursorScene = new THREE.Scene();
        const colorScene = new THREE.Scene();
        const pickScene = new THREE.Scene();
        const cube = new ColorSpaceCube(
            colorScene,
            pickScene,
            params.space_embedding,
            params.color_embedding
        );
        screenScene.add(cursor_plane);
        screenScene.add(color_plane);

        const controller = cameraController(camera);

        return new ColorSpace({
            ...params,
            saved_color: params.color,
            renderer,
            screenScene,
            camera: camera,
            cameraController: controller,
            pickScene,
            pickTarget,
            cube,
            cursors: [],
            slice_direction: params.slice_direction,
            spring: spring({
                theta: controller.theta,
                phi: controller.phi,
                vertical_slice: 1,
                horizontal_slice: 1,
                slice_start: 0,
            }),
            cursorScene,
            cursorTarget,
            colorScene,
            colorTarget,
            screenCamera: orthoCamera,
        });
    }

    set({ color, saved_color, me = false }: { color: Vec3; saved_color?: Vec3, me?: boolean }) {
        if (near(color, this.color)) {
            return;
        }
        if (saved_color) {
            this.saved_color.copy(saved_color);
        }
        this.color = color.clone();
        if (!me) {
            this.update_slice();
        }
    }

    set_slice(slice_direction: string) {
        this.slice_direction = slice_direction;
        if (slice_direction === "vertical") {
            this.spring.set("theta", 0);
            this.spring.set("horizontal_slice", 1);
            this.spring.set("vertical_slice", 0.5);
        }
        else {
            this.spring.set("theta", Math.PI / 2);
            this.spring.set("horizontal_slice", this.color.y);
            this.spring.set("vertical_slice", 1);
        }
        this.update_slice();
    }

    update_slice(force = false) {
        if (this.slice_direction === "horizontal") {
            this.spring.set("horizontal_slice", this.color.y, force);
        }
        else if (this.slice_direction === "vertical") {
            this.spring.set("horizontal_slice", 1, force);
            this.spring.set("vertical_slice", 0.5, force);
            this.spring.set("phi", (-this.color.x + 0.75) * Math.PI * 2, force);
        }
    }

    render(cursors?: CursorSpec[]) {
        this.spring.update();
        this.cameraController.phi = this.spring.get("phi");
        this.cameraController.theta = this.spring.get("theta");
        this.cube.set(this.spring.get("horizontal_slice"), -(this.cameraController.phi / Math.PI) - 0.5, this.spring.get("vertical_slice"))

        this.cameraController.update();
        setCursors(this.cursors, {
            fallback: this.color,
            scene: this.cursorScene,
            embedding: this.cube.space_embedding.embed,
            specs: cursors?.map((spec) => {
                if (this.slice_direction === "horizontal") {
                    const delta = Math.abs(this.color.y - spec.pos.y)
                    return {
                        ...spec,
                        opacity: cursor_fade(delta)
                    }
                }
                else {
                    let delta = Math.abs(this.color.x - spec.pos.x)
                    delta = Math.min(delta, Math.abs(delta - 0.5));
                    return {
                        ...spec,
                        opacity: cursor_fade(delta)
                    }
                }
                return spec;
            }),
        })
        const renderer = this.renderer;

        renderer.setRenderTarget(this.cursorTarget);
        renderer.clear();
        renderer.render(this.cursorScene, this.camera);

        renderer.setRenderTarget(this.colorTarget);
        renderer.clear();
        renderer.render(this.colorScene, this.camera);

        renderer.setRenderTarget(null);
        renderer.clear();
        renderer.render(this.screenScene, this.screenCamera);
    }

    mouse_position(e: MouseEvent) {
        const rect = this.canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = rect.bottom - e.clientY;
        return vec2(x, y);
    }

    mouse_select(e: MouseEvent) {
        const mouse = this.mouse_position(e);
        const picked = this.pick(mouse.x, mouse.y);
        if (picked) {
            this.set({ color: picked, me: true });
        } else {
            this.set({ color: this.saved_color, me: true });
        }
        const mouseDown = e.buttons === 1;
        if (this.select_position && !mouseDown && near2(this.select_position, mouse, 1)) {
            this.onChange?.({
                color: this.color,
                saved_color: this.color,
            });
            if (picked) {
                this.saved_color = picked.clone();
            }
        }
        if (mouseDown) {
            this.cameraController.on_move(vec3(e.movementX, e.movementY, 0.0));
            if (!this.select_position) {
                this.select_position = mouse;
            }
            this.spring.set("theta", this.cameraController.theta, true);
            this.spring.set("phi", this.cameraController.phi, true);
        }
        else {
            this.onChange?.({
                color: this.color,
            });
            this.select_position = undefined;
        }
    }

    pick(x: number, y: number): Vec3 | undefined {
        const renderer = this.renderer;

        renderer.setRenderTarget(this.pickTarget);
        renderer.clear();
        renderer.render(this.pickScene, this.camera);
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

        const colorPosition = wrapAxis(AXIS.X, vec3(pixelBuffer[0], pixelBuffer[1], pixelBuffer[2]));
        renderer.setRenderTarget(null);
        return colorPosition;
    }
}


function cursor_fade(delta: number) {
    return Math.max(0.5 - (delta * 3) ** 3, Math.min(1.5 - delta * 40, 1), 0.1);
}