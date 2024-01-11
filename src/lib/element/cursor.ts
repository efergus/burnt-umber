import type { Color } from "$lib/color";
import type { Vec3 } from "$lib/geometry/vec";
import * as THREE from 'three';


export interface CursorSpec {
    pos: Vec3;
    color?: Color;
    opacity?: number;
    size?: number;
}

export class Cursor {
    mesh: THREE.Mesh<THREE.BufferGeometry, THREE.MeshBasicMaterial>;
    constructor(scene: THREE.Scene, color: THREE.ColorRepresentation, size = 1) {
        const geometry = new THREE.SphereGeometry(0.1 * size, 32, 16);
        const material = new THREE.MeshBasicMaterial({
            color,
            transparent: true,
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
    size?: number,
    embedding?: (i: Vec3) => Vec3,
    specs?: CursorSpec[],
}

export function setCursors(cursors: Cursor[], { fallback, embedding, scene, size, specs }: SetCursorsOptions) {
    if (specs) {
        for (let i = 0; i < specs.length; i++) {
            const color = new THREE.Color(specs[i].color?.to_hex() ?? 0);
            if (!cursors[i]) {
                cursors.push(new Cursor(scene, color, 1));
            }
            const pos = specs[i].pos;
            const position = embedding?.(pos) ?? pos;
            cursors[i].mesh.position.copy(position);
            cursors[i].mesh.scale.setScalar((specs[i].size ?? 1) * (size ?? 1));
            cursors[i].mesh.visible = true;
            cursors[i].mesh.material.color = color;
            cursors[i].mesh.material.opacity = specs[i].opacity ?? 1;
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