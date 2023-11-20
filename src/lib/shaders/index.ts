import vertsrc from '$lib/shaders/vert.glsl?raw';
import fragsrc from '$lib/shaders/frag.glsl?raw';
import funcsrc from '$lib/shaders/func.glsl?raw';

export function vert(...str: string[]): string {
    return vertsrc.replace('// FUNCTIONS', funcsrc).replace('// REPLACE', str.join('\n'));
}

export function frag(...str: string[]): string {
    return fragsrc.replace('// FUNCTIONS', funcsrc).replace('// REPLACE', str.join('\n'));
}
