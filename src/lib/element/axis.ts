import { frag, vert } from '$lib/shaders';
import { pick_shader, type Embedding, black_shader } from '$lib/shaders/embed';
import * as THREE from 'three';

export enum AXIS {
    X = 0,
    Y = 1,
    Z = 2
}

export interface Axis extends ColorElement {
    color_embedding: Embedding;
    input_pos: THREE.Vector3;
}

export class Axis {
    static new(
        canvas: HTMLCanvasElement,
        // space_embedding: Embedding,
        color_embedding: Embedding,
        axis: AXIS
    ): Axis {
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

        const geometry = new THREE.PlaneGeometry(1, 1, 32, 32);
        const cursor_geometry = new THREE.SphereGeometry(0.05, 8, 8);
        const boundingBox = new THREE.Box3().setFromObject(new THREE.Mesh(geometry));
        const embedMatrix = new THREE.Matrix4().makeTranslation(boundingBox.min.multiplyScalar(-1));

        const material = new THREE.ShaderMaterial({
            vertexShader: vert(),
            fragmentShader: frag(color_embedding.shader),
            uniforms: {
                embedMatrix: { value: embedMatrix }
            }
        });
        const pick_material = new THREE.ShaderMaterial({
            vertexShader: vert(),
            fragmentShader: frag(pick_shader),
            uniforms: {
                embedMatrix: { value: embedMatrix },
                tag: { value: 1 }
            }
        });
        const cursor_material = new THREE.ShaderMaterial({
            vertexShader: vert(),
            fragmentShader: frag(black_shader),
            uniforms: {
                embedMatrix: { value: new THREE.Matrix4() }
            }
        });

        const mesh = new THREE.Mesh(geometry, material);
        const pick_mesh = new THREE.Mesh(geometry.clone(), pick_material);
        const cursor_mesh = new THREE.Mesh(cursor_geometry, cursor_material);

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

        pick_mesh.scale.copy(mesh.scale);
        pick_mesh.position.copy(mesh.position);

        scene.add(mesh, cursor_mesh);

        return {
            ortho: true,
            color_embedding,
            input_pos: new THREE.Vector3(),
            on_input_change(pos: THREE.Vector3) {
                this.input_pos.copy(pos);
                if (axis == AXIS.X) {
                    const embedMatrix = new THREE.Matrix4().makeTranslation(
                        boundingBox.min.x,
                        pos.y,
                        pos.z
                    );
                    embedMatrix.multiply(new THREE.Matrix4().makeScale(1, 0, 0));
                    mesh.material.uniforms.embedMatrix.value = embedMatrix;
                    cursor_mesh.position.x = pos.x - 0.5;
                } else if (axis == AXIS.Y) {
                    const embedMatrix = new THREE.Matrix4().makeTranslation(
                        pos.x,
                        boundingBox.min.y,
                        pos.z
                    );
                    embedMatrix.multiply(new THREE.Matrix4().makeScale(0, 1, 0));
                    mesh.material.uniforms.embedMatrix.value = embedMatrix;
                    cursor_mesh.position.y = pos.y - 0.5;
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
                    cursor_mesh.position.x = pos.z - 0.5;
                }
            }
        };
    }
}
