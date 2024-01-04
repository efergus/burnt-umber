import type { Vec3 } from "$lib/geometry/vec";
import { frag, vert } from "$lib/shaders";
import { black_shader } from "$lib/shaders/embed";
import * as THREE from 'three';


export interface CursorSpec {
    pos: Vec3;
    color?: Vec3;
    size?: number;
}

export class Cursor {
    mesh: THREE.Mesh;
    constructor(scene: THREE.Scene, size = 1) {
        const geometry = new THREE.SphereGeometry(0.1 * size, 16, 8);
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

interface SetCursorsOptions {
    fallback: Vec3,
    scene: THREE.Scene,
    embedding?: (i: Vec3) => Vec3,
    specs?: CursorSpec[],
}

export function setCursors(cursors: Cursor[], { fallback, embedding, scene, specs }: SetCursorsOptions) {
    if (specs) {
        for (let i = 0; i < specs.length; i++) {
            if (!cursors[i]) {
                cursors.push(new Cursor(scene));
            }
            const pos = specs[i].pos;
            const position = embedding?.(pos) ?? pos;
            cursors[i].mesh.position.copy(position);
            cursors[i].mesh.scale.setScalar(specs[i].size ?? 1);
            cursors[i].mesh.visible = true;
        }
        for (let i = specs.length; i < cursors.length; i++) {
            cursors[i].mesh.visible = false;
        }
    }
    else {
        const position = embedding?.(fallback) ?? fallback;
        for (let i = 0; i < cursors.length; i++) {
            cursors[i].mesh.position.copy(position);
        }
    }
}