import { vec3, type Vec3 } from '$lib/geometry/vec';
import * as THREE from 'three';
import { vert } from '.';

export enum EmbeddingName {
    cylindrical,
    cartesian,
    hsv,
}

export interface Embedding {
    name: EmbeddingName,
    shader: string;
    embed?: (pos: THREE.Vector3) => THREE.Vector3;
    invert?: (pos: THREE.Vector3) => THREE.Vector3;
    center?: Vec3;
}

export interface CPUEmbedding extends Embedding {
    embed: (pos: THREE.Vector3) => THREE.Vector3;
}

export interface InvertibleEmbedding extends Embedding {
    invert: (pos: THREE.Vector3) => THREE.Vector3;
}

export type CompleteEmbedding = CPUEmbedding & InvertibleEmbedding;

export const rgb_shader = 'fragColor.xyz = fragColor.xyz;';
export const hsv_shader = 'fragColor.xyz = hsv2rgb(fragColor.xzy);';
export const step_shader = 'fragColor.xyz = vec3(step(fragColor.x, 0.5));';
export const pick_shader = 'fragColor.w = tag;';
export const white_shader = 'fragColor.xyz = vec3(1.0);';
export const grey_shader = 'fragColor.xyz = vec3(0.5);';
export const black_shader = 'fragColor.xyz = vec3(0.0);';
export const tDiffuse_shader = 'fragColor.xyz = texture2D(tDiffuse, vUv).xyz;';
export const clipOutOfGamut_shader = 'if(out_of_gamut(fragColor.xyz)) discard;';

export const embed_shader = 'vertPosition = vertEmbed;';
export const cylindrical_shader = 'vertPosition = cylindricalToCartesian(vertPosition);';
export const inverse_cylindrical_shader = 'vertPosition = cartesianToCylindrical(vertPosition);';
export const inverse_cylindrical_frag_shader =
    'fragColor.xyz = cartesianToCylindrical(fragColor.xyz);';

export function identity<T = Vec3>(pos: T) {
    return pos;
}

export function cylindricalToCartesian(pos: THREE.Vector3) {
    const theta = pos.x;
    const r = pos.z;
    const y = pos.y;
    return new THREE.Vector3(
        -r * Math.cos(theta * Math.PI * 2.0),
        y,
        r * Math.sin(theta * Math.PI * 2.0)
    );
}

export function cartestianToCylindrical(pos: THREE.Vector3) {
    const x = pos.x;
    const y = pos.y;
    const z = pos.z;
    const theta = Math.atan2(z, x) / (Math.PI * 2.0);
    const r = Math.sqrt(x * x + z * z);
    return new THREE.Vector3(theta, y, r);
}

export const cylindrical: CompleteEmbedding = {
    name: EmbeddingName.cylindrical,
    shader: vert(embed_shader, cylindrical_shader),
    embed: cylindricalToCartesian,
    invert: cartestianToCylindrical,
    center: vec3(0, 0.5, 0),
};

export const cartesian: CompleteEmbedding = {
    name: EmbeddingName.cartesian,
    shader: vert(),
    embed: identity,
    invert: identity,
    center: vec3(0.5, 0.5, 0.5),
}

export const hsv: Embedding = {
    name: EmbeddingName.hsv,
    shader: hsv_shader
};

export default {
    cylindrical,
    hsv
};
