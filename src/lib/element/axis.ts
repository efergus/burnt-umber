import { frag, vert } from '$lib/shaders';
import type { Embedding } from '$lib/shaders/embed';
import * as THREE from 'three';
import type { ColorElement } from '.';
import { near, vec3x, type Vec3, vec3y } from '$lib/geometry/vec';
import { clamp } from '$lib/math';
import { setCursors, type CursorSpec, Cursor } from './cursor';

export enum AXIS {
    X = 0,
    Y = 1,
    Z = 2
}

export interface ColorState {
    color: Vec3;
    saved_color?: Vec3;
};

export interface Axis extends ColorElement {
    color_embedding: Embedding;
    color: THREE.Vector3;
    saved_color: THREE.Vector3;

    onChange?: (params: ColorState) => void;
    set(params: ColorState): void;

    pick(x: number, y: number): THREE.Vector3;
    render(cursors?: CursorSpec[]): void;

    mouse_position(e: MouseEvent): { x: number; y: number };
    mouse_select(e: MouseEvent): void;
    restore(): void;
}

export class Axis {
    static new(
        canvas: HTMLCanvasElement,
        // space_embedding: Embedding,
        color_embedding: Embedding,
        axis: AXIS,
        onChange?: (params: ColorState) => void
    ): Axis {
        const rect = canvas.getBoundingClientRect();
        const renderer = new THREE.WebGLRenderer({ canvas, antialias: true, alpha: true });
        renderer.setSize(rect.width, rect.height);
        renderer.autoClear = false;

        const scene = new THREE.Scene();
        const aspect = rect.width / rect.height;
        const width = aspect > 1 ? 1 : aspect;
        const height = aspect > 1 ? 1 / aspect : 1;
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

        const geometry = new THREE.PlaneGeometry(1, 1, 32, 32);
        const boundingBox = new THREE.Box3().setFromObject(new THREE.Mesh(geometry));
        const embedMatrix = new THREE.Matrix4().makeTranslation(boundingBox.min.multiplyScalar(-1));

        const material = new THREE.ShaderMaterial({
            vertexShader: vert(),
            fragmentShader: frag(color_embedding.shader),
            uniforms: {
                embedMatrix: { value: embedMatrix }
            }
        });

        const mesh = new THREE.Mesh(geometry, material);

        const scale = Math.min(width, height);
        if (axis == AXIS.X) {
            mesh.scale.y = scale;
        } else if (axis == AXIS.Y) {
            mesh.scale.x = scale;
        } else if (axis == AXIS.Z) {
            mesh.scale.y = scale;
        } else {
            throw new Error(`Unknown axis ${axis}`);
        }

        scene.add(mesh);

        const cursor = new Cursor(scene);
        const cursor_objs = [cursor];

        return {
            color: new THREE.Vector3(0, 0, 0),
            saved_color: new THREE.Vector3(0, 0, 0),
            color_embedding,
            onChange,
            on_input_change(pos: THREE.Vector3) {
                if (axis == AXIS.X) {
                    const embedMatrix = new THREE.Matrix4().makeTranslation(
                        boundingBox.min.x,
                        pos.y,
                        pos.z
                    );
                    embedMatrix.multiply(new THREE.Matrix4().makeScale(1, 0, 0));
                    mesh.material.uniforms.embedMatrix.value = embedMatrix;
                    // cursor_mesh.position.x = pos.x - 0.5;
                } else if (axis == AXIS.Y) {
                    const embedMatrix = new THREE.Matrix4().makeTranslation(
                        pos.x,
                        boundingBox.min.y,
                        pos.z
                    );
                    embedMatrix.multiply(new THREE.Matrix4().makeScale(0, 1, 0));
                    mesh.material.uniforms.embedMatrix.value = embedMatrix;
                    // cursor_mesh.position.y = pos.y - 0.5;
                } else if (axis == AXIS.Z) {
                    let embedMatrix = new THREE.Matrix4().makeTranslation(boundingBox.min.x, 0, 0);
                    embedMatrix = new THREE.Matrix4()
                        .makeRotationY(-Math.PI / 2)
                        .multiply(embedMatrix);
                    embedMatrix = new THREE.Matrix4().makeScale(0, 0, 1).multiply(embedMatrix);
                    embedMatrix = new THREE.Matrix4()
                        .makeTranslation(pos.x, pos.y, 0)
                        .multiply(embedMatrix);
                    mesh.material.uniforms.embedMatrix.value = embedMatrix;
                    // cursor_mesh.position.x = pos.z - 0.5;
                }
            },
            set({ color, saved_color }) {
                if (saved_color && !near(this.saved_color, saved_color)) {
                    this.saved_color.copy(saved_color);
                }
                if (near(this.color, color)) {
                    return;
                }
                this.color.copy(color);
                this.on_input_change(color);
            },
            render(cursors?: CursorSpec[]) {
                setCursors(cursor_objs, {
                    fallback: this.color,
                    scene,
                    specs: cursors,
                    embedding: {
                        [AXIS.X]: (v: Vec3) => vec3x(v.x - 0.5),
                        [AXIS.Y]: (v: Vec3) => vec3y(v.y - 0.5),
                        [AXIS.Z]: (v: Vec3) => vec3x(v.z - 0.5),
                    }[axis]
                })
                renderer.clear();
                renderer.render(scene, camera);
            },
            pick(x: number, y: number) {
                const px = clamp(x / rect.width);
                const py = clamp(y / rect.height);
                const color = this.color.clone();
                if (axis === AXIS.Y) {
                    color.y = py;
                } else {
                    color.setComponent(axis, px);
                }
                return color;
            },
            mouse_position(e: MouseEvent) {
                const rect = canvas.getBoundingClientRect();
                const x = e.clientX - rect.left;
                const y = rect.bottom - e.clientY;
                return { x, y };
            },
            mouse_select(e: MouseEvent) {
                const { x, y } = this.mouse_position(e);
                const picked = this.pick(x, y);
                const selecting = e.buttons === 1;
                if (selecting) {
                    this.set({ color: picked, saved_color: picked });
                    this.onChange?.({ color: picked, saved_color: picked.clone() });
                    console.log('selecting', ...picked);
                }
                else {
                    this.set({ color: picked });
                    this.onChange?.({ color: picked });
                }
            },
            restore() {
                this.set({ color: this.saved_color });
                this.onChange?.({ color: this.saved_color });
            }
        };
    }
}
