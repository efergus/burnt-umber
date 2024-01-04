import * as THREE from 'three';

export type Vec2 = THREE.Vector2;
export type Vec3 = THREE.Vector3;

export type Mat3 = THREE.Matrix3;
export type Mat4 = THREE.Matrix4;

export function vec3(x?: number, y?: number, z?: number): THREE.Vector3 {
    return new THREE.Vector3(x, y, z);
}

export function vec3x(x: number) {
    return vec3(x, 0, 0);
}

export function vec3y(y: number) {
    return vec3(0, y, 0);
}

export function vec3z(z: number) {
    return vec3(0, 0, z);
}

export function vec2(x?: number, y?: number): THREE.Vector2 {
    return new THREE.Vector2(x, y);
}

export function dot(a: THREE.Vector3, b: THREE.Vector3): number {
    const tmp = a.clone();
    return tmp.dot(b);
}

export function cross(a: THREE.Vector3, b: THREE.Vector3): THREE.Vector3 {
    const tmp = a.clone();
    return tmp.cross(b);
}

export function near(a: Vec3, b: Vec3, eps = 1e-6): boolean {
    return a.distanceTo(b) < eps;
}

export const zeros = vec3(0, 0, 0);
export const ones = vec3(1, 1, 1);

export const unit = {
    x: vec3(1, 0, 0),
    y: vec3(0, 1, 0),
    z: vec3(0, 0, 1)
};

export const units = [unit.x, unit.y, unit.z];
