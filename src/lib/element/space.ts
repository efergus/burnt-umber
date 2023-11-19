import { frag, vert } from "$lib/shaders";
import { pick_shader, type Embedding, black_shader } from "$lib/shaders/embed";
import * as THREE from 'three';

interface Space extends ColorElement {
    space_embedding: Embedding;
    color_embedding: Embedding;
    input_pos: THREE.Vector3;

    on_input_change(pos: THREE.Vector3): void;
}

export function space(space_embedding: Embedding, color_embedding: Embedding, tag: number): Space {

    const geometry = new THREE.BoxGeometry(1, 1, 1, 32, 8, 8);
    const cursor_geometry = new THREE.SphereGeometry(0.1, 8, 8);
    const boundingBox = new THREE.Box3().setFromObject(new THREE.Mesh(geometry));
    const embedMatrix = new THREE.Matrix4();
    embedMatrix.makeTranslation(boundingBox.min.multiplyScalar(-1));
    
    const material = new THREE.ShaderMaterial({
        vertexShader: vert(space_embedding.shader),
        fragmentShader: frag(color_embedding.shader),
        uniforms: {
            embedMatrix: { value: embedMatrix },
        }
    });
    const pick_material = new THREE.ShaderMaterial({
        vertexShader: vert(space_embedding.shader),
        fragmentShader: frag(pick_shader),
        uniforms: {
            embedMatrix: { value: embedMatrix },
            tag: { value: tag },
        }
    });
    const cursor_material = new THREE.ShaderMaterial({
        vertexShader: vert(),
        fragmentShader: frag(black_shader),
        uniforms: {
            embedMatrix: { value: new THREE.Matrix4() },
        }
    });

    const mesh = new THREE.Mesh(geometry, material);
    const pick_mesh = new THREE.Mesh(geometry.clone(), pick_material);
    const cursor_mesh = new THREE.Mesh(cursor_geometry, cursor_material);

    return {
        meshes: [mesh, cursor_mesh],
        pick_meshes: [pick_mesh],
        space_embedding,
        color_embedding,
        input_pos: new THREE.Vector3(),
        on_input_change(pos: THREE.Vector3) {
            this.input_pos.copy(pos);
            cursor_mesh.position.copy(this.space_embedding.embed!(pos));
        }
    }
}