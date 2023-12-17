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
    slice: number;

    on_input_change(pos: THREE.Vector3, me?: boolean): void;
    set_slice(slice: number): void;
}

export function space(space_embedding: Embedding, color_embedding: Embedding, tag: number): Space {
    const geometry = new THREE.BoxGeometry(1, 1, 1, 64, 8, 8);
    const plane_geometry = new THREE.PlaneGeometry(4, 4);
    const cursor_geometry = new THREE.SphereGeometry(0.1, 8, 8);
    const boundingBox = new THREE.Box3().setFromObject(new THREE.Mesh(geometry));
    const embedMatrix = new THREE.Matrix4();
    embedMatrix.makeTranslation(boundingBox.min.multiplyScalar(-1));

    const material = new THREE.ShaderMaterial({
        // side: THREE.DoubleSide,
        vertexShader: vert(embed_shader, space_embedding.shader),
        fragmentShader: definitions("USE_CLIP_PLANE") + frag(color_embedding.shader),
        uniforms: {
            clipPlane: { value: new THREE.Vector4(0, 0, 1, 1) },
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
        side: THREE.DoubleSide,
        vertexShader: vert(embed_shader, space_embedding.shader),
        fragmentShader: definitions("USE_CLIP_PLANE") + frag(pick_shader),
        uniforms: {
            clipPlane: { value: new THREE.Vector4(0, 0, 1, 1) },
            embedMatrix: { value: embedMatrix },
            tag: { value: tag }
        }
    });
    const pick_plane_material = new THREE.ShaderMaterial({
        side: THREE.DoubleSide,
        vertexShader: vert(embed_shader),
        fragmentShader: frag(inverse_cylindrical_frag_shader, pick_shader, clipOutOfGamut_shader),
        uniforms: {
            embedMatrix: { value: new THREE.Matrix4() },
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
    const pick_plane_mesh = new THREE.Mesh(plane_geometry.clone(), pick_plane_material);
    const cursor_mesh = new THREE.Mesh(cursor_geometry, cursor_material);

    plane_mesh.visible = false;

    const clip = (pos: THREE.Vector3, slice: number) => {
        // if (slice >= 1.0) {
        //     const clip_plane = new THREE.Vector4(0, 0, 1, 2);
        //     material.uniforms.clipPlane.value = clip_plane;
        //     pick_material.uniforms.clipPlane.value = clip_plane;
        //     plane_mesh.visible = false;
        //     pick_plane_mesh.visible = false;
        //     return;
        // }
        // const rotation = pos.x * Math.PI * 2;
        // const plane_embedding = new THREE.Matrix4().makeRotationY(rotation);
        // plane_mesh.material.uniforms.embedMatrix.value = plane_embedding;
        // pick_plane_mesh.material.uniforms.embedMatrix.value = plane_embedding;
        // plane_mesh.visible = true;
        // pick_plane_mesh.visible = true;

        // const plane_direction = new THREE.Vector3(0, 0, -1).applyAxisAngle(new THREE.Vector3(0, 1, 0), rotation);
        // const clip_plane = new THREE.Vector4(...plane_direction, slice);
        // material.uniforms.clipPlane.value = clip_plane;
        // pick_material.uniforms.clipPlane.value = clip_plane;

        const embedding = new THREE.Matrix4().makeTranslation(pos.x, 0, 0);
        // const embedding = new THREE.Matrix4().makeScale(0.5, 1, 1);
        embedding.multiply(new THREE.Matrix4().makeScale(0.5, 1, 1));
        embedding.multiply(embedMatrix);
        material.uniforms.embedMatrix.value = embedding;
        pick_material.uniforms.embedMatrix.value = embedding;
    }

    return {
        meshes: [mesh, cursor_mesh],
        pick_meshes: [pick_mesh],
        space_embedding,
        color_embedding,
        input_pos: new THREE.Vector3(),
        slice: 1,
        on_input_change(pos: THREE.Vector3, me?: boolean) {
            this.input_pos.copy(pos);
            const embedded_pos = this.space_embedding.embed!(pos);
            cursor_mesh.position.copy(embedded_pos);
            // console.log("Cursor pos:", pos);

            if (!me) {
                clip(pos, this.slice);
            }
        },
        set_slice(slice: number) {
            this.slice = slice;
            clip(this.input_pos, slice)
        }
    };
}
