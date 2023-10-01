type num = number;
export type Point = [number, number, number] | number[];

export interface Mesh<T extends number[]> {
    points: { point: Point; data: T }[];
}

export const createShader = (
    gl: WebGL2RenderingContext,
    type: any,
    source: string
): WebGLShader => {
    const shader = gl.createShader(type);
    if (shader === null) {
        throw Error('Failed to create shader');
    }
    gl.shaderSource(shader, source);
    gl.compileShader(shader);
    if (!gl.getShaderParameter(shader, gl.COMPILE_STATUS)) {
        throw Error(`Failed to compile shader: ${gl.getShaderInfoLog(shader)}`);
    }
    return shader;
};

export function createProgram(
    gl: WebGLRenderingContext,
    vertexShader: WebGLShader,
    fragmentShader: WebGLShader
): WebGLProgram {
    const program = gl.createProgram();
    if (!program) {
        throw Error('Failed to create shader');
    }
    gl.attachShader(program, vertexShader);
    gl.attachShader(program, fragmentShader);
    gl.linkProgram(program);
    if (!gl.getProgramParameter(program, gl.LINK_STATUS)) {
        throw Error(`Failed to link shader: ${gl.getProgramInfoLog(program)}`);
    }
    return program;
}

export const plane = (x = 1, y = 1) => {
    return [0, 0, 0, x, y, 0, x, 0, 0, 0, 0, 0, 0, y, 0, x, y, 0];
};

export const centerplane = (x = 1, y = 1) => {
    return [-x, -y, 0, x, y, 0, x, -y, 0, -x, -y, 0, -x, y, 0, x, y, 0];
};
