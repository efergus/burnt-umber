import { near, vec3, type Vec3 } from '$lib/geometry/vec';
import { definitions, frag, vert } from '$lib/shaders';
import {
    pick_shader,
    type Embedding,
    black_shader,
    embed_shader,
} from '$lib/shaders/embed';
import * as THREE from 'three';
import { cameraController, type CameraController } from './controller';
import type { ColorElement } from '.';
import type { ColorState } from './axis';

export interface CursorSpec {
    pos: Vec3;
    color?: Vec3;
    size?: number;
}

export interface Space extends ColorElement {
    space_embedding: Embedding;
    color_embedding: Embedding;
    input_pos: THREE.Vector3;
    slice: number;

    on_input_change(pos: THREE.Vector3, me?: boolean): void;
    set_slice(slice: number): void;
}

class Cursor {
    mesh: THREE.Mesh;
    constructor(scene: THREE.Scene) {
        const geometry = new THREE.SphereGeometry(0.1, 16, 8);
        const material = new THREE.ShaderMaterial({
            vertexShader: vert(),
            fragmentShader: frag(black_shader),
            uniforms: {
                embedMatrix: { value: new THREE.Matrix4() }
            }
        });
        this.mesh = new THREE.Mesh(geometry, material);

        scene.add(this.mesh);
    }

    set(pos: THREE.Vector3) {
        this.mesh.position.copy(pos);
    }
}

class ColorSpaceCube {
    mesh: THREE.Mesh;
    pick_mesh: THREE.Mesh;
    space_embedding: Embedding;
    color_embedding: Embedding;

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
            // side: THREE.DoubleSide,
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
    }
}

type WithoutMethods<T> = {
    // eslint-disable-next-line @typescript-eslint/ban-types
    [K in keyof T as T[K] extends Function ? never : K]: T[K];
};

export interface ColorSpaceParams {
    canvas: HTMLCanvasElement;
    color: Vec3;
    space_embedding: Embedding;
    color_embedding: Embedding;
    slice: number;

    onChange?: (state: ColorState) => void;
}

export class ColorSpace {
    canvas: HTMLCanvasElement;
    color: Vec3;
    saved_color: Vec3;
    renderer: THREE.WebGLRenderer;
    screenScene: THREE.Scene;
    camera: THREE.PerspectiveCamera;
    cameraController: CameraController;
    pickScene: THREE.Scene;
    pickTarget: THREE.WebGLRenderTarget;

    cube: ColorSpaceCube;
    cursors: Cursor[];

    onChange?: (color: ColorState) => void;

    constructor({
        color,
        saved_color,
        canvas,
        renderer,
        screenScene,
        camera: screenCamera,
        cameraController,
        pickScene,
        pickTarget,
        cube,
        cursors,
        onChange
    }: WithoutMethods<ColorSpace>) {
        this.canvas = canvas;
        this.color = color;
        this.saved_color = saved_color;
        this.renderer = renderer;
        this.screenScene = screenScene;
        this.camera = screenCamera;
        this.cameraController = cameraController;
        this.pickScene = pickScene;
        this.pickTarget = pickTarget;

        this.cube = cube;
        this.cursors = cursors;

        this.onChange = onChange;

        canvas.addEventListener('mousemove', (e) => {
            this.mouse_select(e);
        });
        canvas.addEventListener('mousedown', (e) => {
            this.mouse_select(e);
        });
        canvas.addEventListener(
            'wheel',
            (e) => {
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
        renderer.setPixelRatio(1);
        renderer.autoClear = false;

        const camera = new THREE.PerspectiveCamera(40, 1, 0.01, 100);
        const orthoCamera = new THREE.OrthographicCamera(-1, 1, 1, -1, 0.1, 100);
        orthoCamera.position.z = 1;
        orthoCamera.lookAt(0, 0, 0);

        // TODO: do 1x1 and scissor?
        const pickTarget = new THREE.WebGLRenderTarget(rect.width, rect.height, {
            format: THREE.RGBAFormat,
            type: THREE.FloatType
        });
        pickTarget.texture.generateMipmaps = false;

        const screenScene = new THREE.Scene();
        const pickScene = new THREE.Scene();
        const cube = new ColorSpaceCube(
            screenScene,
            pickScene,
            params.space_embedding,
            params.color_embedding
        );

        const cursor = new Cursor(screenScene);

        return new ColorSpace({
            ...params,
            saved_color: params.color,
            renderer,
            screenScene,
            camera: camera,
            cameraController: cameraController(camera),
            pickScene,
            pickTarget,
            cube,
            cursors: [cursor],
        });
    }

    set({ color, saved_color }: { color: Vec3; saved_color?: Vec3 }) {
        if (near(color, this.color)) {
            return;
        }
        if (saved_color) {
            this.saved_color.copy(saved_color);
        }
        this.color = color.clone();
        // const position = this.cube.space_embedding.embed!(color);
        // this.saved_color = saved_color;
    }

    render(cursors?: CursorSpec[]) {
        if (cursors) {
            for (let i = 0; i < cursors.length; i++) {
                if (!this.cursors[i]) {
                    this.cursors.push(new Cursor(this.screenScene));
                }
                const position = this.cube.space_embedding.embed!(cursors[i].pos);
                this.cursors[i].mesh.position.copy(position);
                this.cursors[i].mesh.scale.setScalar(cursors[i].size ?? 1);
                this.cursors[i].mesh.visible = true;
            }
            for (let i = cursors.length; i < this.cursors.length; i++) {
                this.cursors[i].mesh.visible = false;
            }
        }
        else if (this.cursors.length) {
            const position = this.cube.space_embedding.embed!(this.color);
            this.cursors[0].mesh.position.copy(position)
        }
        const renderer = this.renderer;
        renderer.setRenderTarget(null);
        renderer.clear();
        renderer.render(this.screenScene, this.camera);
    }

    mouse_position(e: MouseEvent) {
        const rect = this.canvas.getBoundingClientRect();
        const x = e.clientX - rect.left;
        const y = rect.bottom - e.clientY;
        return { x, y };
    }

    mouse_select(e: MouseEvent) {
        const { x, y } = this.mouse_position(e);
        const picked = this.pick(x, y);
        if (picked) {
            this.set({ color: picked });
        } else {
            this.set({ color: this.saved_color });
        }
        this.onChange?.({
            color: this.color,
            saved_color: this.saved_color,
        });
        const selecting = e.buttons === 1;
        if (selecting) {
            if (picked) {
                this.saved_color = picked.clone();
            }
            this.cameraController.on_move(vec3(e.movementX, e.movementY, 0.0));
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

        const colorPosition = new THREE.Vector3(pixelBuffer[0], pixelBuffer[1], pixelBuffer[2]);
        renderer.setRenderTarget(null);
        return colorPosition;
    }
}
