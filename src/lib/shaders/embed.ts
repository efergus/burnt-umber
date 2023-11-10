import * as THREE from 'three';

export const hsv_shader = "fragColor = hsv2rgb(fragColor.xzy);"
export const step_shader = "fragColor = vec3(step(fragColor.x, 0.5));"

export const cylindrical_shader = "vertPosition = cylindricalToCartesian(vertPosition);"


export function cylindricalToCartesian(pos: THREE.Vector3) {
    let theta = pos.x;
    let r = pos.z;
    let y = pos.y;
    return new THREE.Vector3(- r * Math.cos(theta * Math.PI * 2.0), y, r * Math.sin(theta * Math.PI * 2.0));
}