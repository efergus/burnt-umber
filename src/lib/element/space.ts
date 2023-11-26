import { definitions, frag, vert } from '$lib/shaders';
import {
    pick_shader,
    type Embedding,
    black_shader,
    tDiffuse_shader,
    embed_shader,
    clipOutOfGamut_shader,
    inverse_cylindrical_frag_shader
} from '$lib/shaders/embed';
import * as THREE from 'three';

export interface Space extends ColorElement {
    space_embedding: Embedding;
    color_embedding: Embedding;
    input_pos: THREE.Vector3;

    on_input_change(pos: THREE.Vector3): void;
}

export function space(space_embedding: Embedding, color_embedding: Embedding, tag: number): Space {
    const geometry = new THREE.BoxGeometry(1, 1, 1, 64, 8, 8);
    const plane_geometry = new THREE.PlaneGeometry(4, 4);
    const cursor_geometry = new THREE.SphereGeometry(0.1, 8, 8);
    const boundingBox = new THREE.Box3().setFromObject(new THREE.Mesh(geometry));
    const embedMatrix = new THREE.Matrix4();
    embedMatrix.makeTranslation(boundingBox.min.multiplyScalar(-1));

    const material = new THREE.ShaderMaterial({
        vertexShader: vert(embed_shader, space_embedding.shader),
        fragmentShader: definitions("USE_CLIP_PLANE") + frag(color_embedding.shader),
        uniforms: {
            clipPlane: { value: new THREE.Vector4(0, 0, 1, 0) },
            embedMatrix: { value: embedMatrix }
            // modelViewMatrix: { value: new THREE.Matrix4().makeScale(400, 400, 0) },
            // projectionMatrix: { value: new THREE.Matrix4().makeScale(1, 1, 0.1).multiply(new THREE.Matrix4().makeTranslation(0, 0, 5)) }
        }
    });
    const planeMaterial = new THREE.ShaderMaterial({
        side: THREE.DoubleSide,
        vertexShader: vert(embed_shader),
        fragmentShader: frag(inverse_cylindrical_frag_shader, color_embedding.shader, clipOutOfGamut_shader),
        uniforms: {
            // clipPlane: { value: new THREE.Vector4(0, 0, 1, 0) },
            embedMatrix: { value:  new THREE.Matrix4() }
            // modelViewMatrix: { value: new THREE.Matrix4().makeScale(400, 400, 0) },
            // projectionMatrix: { value: new THREE.Matrix4().makeScale(1, 1, 0.1).multiply(new THREE.Matrix4().makeTranslation(0, 0, 5)) }
        }
    });
    const pick_material = new THREE.ShaderMaterial({
        vertexShader: vert(embed_shader, space_embedding.shader),
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
    const plane_mesh = new THREE.Mesh(plane_geometry, planeMaterial);
    const pick_mesh = new THREE.Mesh(geometry.clone(), pick_material);
    const cursor_mesh = new THREE.Mesh(cursor_geometry, cursor_material);

    return {
        meshes: [mesh, plane_mesh, cursor_mesh],
        pick_meshes: [pick_mesh],
        space_embedding,
        color_embedding,
        input_pos: new THREE.Vector3(),
        on_input_change(pos: THREE.Vector3) {
            this.input_pos.copy(pos);
            const embedded_pos = this.space_embedding.embed!(pos);
            cursor_mesh.position.copy(embedded_pos);

            const plane_embedding = new THREE.Matrix4().makeRotationY(pos.x * Math.PI * 2 + Math.PI / 2);
            plane_mesh.material.uniforms.embedMatrix.value = plane_embedding;

            // const clip_plane = new THREE.Vector4(0, 0, 1, -pos.z);
            // material.uniforms.clipPlane.value = clip_plane;

            embedded_pos.y = 0;
            embedded_pos.normalize();
            const clip_plane = new THREE.Vector4(...embedded_pos, 0);
            material.uniforms.clipPlane.value = clip_plane;
        }
    };
}
