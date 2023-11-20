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
        // space_embedding: Embedding,
        color_embedding: Embedding,
        tag: number,
        axis: AXIS
    ): Axis {
        const geometry = new THREE.PlaneGeometry(1, 1, 32, 32);
        const cursor_geometry = new THREE.SphereGeometry(0.1, 8, 8);
        const boundingBox = new THREE.Box3().setFromObject(new THREE.Mesh(geometry));
        const embedMatrix = new THREE.Matrix4().makeTranslation(boundingBox.min.multiplyScalar(-1));
        console.log(...boundingBox.min);
        // embedMatrix.makeTranslation(boundingBox.min.multiplyScalar(-1));
        if (axis == AXIS.X) {
            // embedMatrix.multiply(new THREE.Matrix4().makeScale(1, 1, 1));
        }

        const material = new THREE.ShaderMaterial({
            vertexShader: vert(),
            fragmentShader: frag(color_embedding.shader),
            uniforms: {
                embedMatrix: { value: embedMatrix }
                // modelViewMatrix: { value: new THREE.Matrix4() },
                // projectionMatrix: { value: new THREE.Matrix4().makeTranslation(0, 0, 5) }
            }
        });
        const pick_material = new THREE.ShaderMaterial({
            vertexShader: vert(),
            fragmentShader: frag(pick_shader),
            uniforms: {
                embedMatrix: { value: embedMatrix },
                tag: { value: tag }
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

        mesh.position.z = 1;

        return {
            meshes: [],
            ortho_meshes: [mesh, cursor_mesh],
            pick_meshes: [],
            color_embedding,
            input_pos: new THREE.Vector3(),
            on_input_change(pos: THREE.Vector3) {
                this.input_pos.copy(pos);
                // cursor_mesh.position.copy(this.space_embedding.embed!(pos));
            }
        };
    }
}
