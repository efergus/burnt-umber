
import ColorJS from 'colorjs.io';
import { wrapAxis, type Vec3, clampVec3, vec3 } from './geometry/vec';
import { AXIS } from './element/axis';

// ColorJS.defaults.precision = 3;

type CFV = (color: Vec3) => [
    number, number, number
];

const srgb_from_vec: CFV = (color) => [
    color.x,
    color.y,
    color.z,
]

const hsv_from_vec: CFV = (color) => {
    const c = clampVec3(wrapAxis(AXIS.X, color));
    return [
        c.x * 360,
        c.z * 100,
        c.y * 100,
    ]
}

const cfvs: Record<string, CFV> = {
    'srgb': srgb_from_vec,
    'hsv': hsv_from_vec,
}

type Space = keyof typeof cfvs;

type VFC = (color: ColorJS) => Vec3;

const vec_from_srgb: VFC = (color) => {
    return vec3(color.srgb.r, color.srgb.g, color.srgb.b)
}

const vec_from_hsv: VFC = (color) => {
    return vec3(color.hsv.h / 360, color.hsv.v / 100, color.hsv.s / 100)
}

const vfcs: Record<Space, VFC> = {
    'srgb': vec_from_srgb,
    'hsv': vec_from_hsv,
}

type ReadableStringParams = {
    space?: string, precision?: number
}

export class Color extends ColorJS {
    input: Vec3;

    constructor(space: Space, value: Vec3) {
        space = space.toLowerCase();
        super(space, cfvs[space](value));
        this.input = value.clone()
    }

    static fromString(str: string) {
        if (str.startsWith("hsv(")) {
            const nums = str.match(/[\d-.]+/g)?.map(x => parseFloat(x) / 100);
            if (nums?.length !== 3) {
                throw new Error(`Parsing error ${str}`)
            }
            return new Color("hsv", vec3(...nums))
        }
        const color = new ColorJS(str);
        const value = vfcs[color.spaceId](color);
        return new Color(color.spaceId, value);
    }

    clone(): this {
        const cloned = super.clone();
        cloned.input = this.input.clone();
        return cloned;
    }

    add_norm(value: Vec3): Color {
        const new_value = this.input.clone().add(value);
        return new Color(this.spaceId, new_value);
    }

    set_norm(value: Vec3) {
        this.setAll(this.space, cfvs[this.spaceId](value))
    }

    get_norm(space?: Space) {
        return vfcs[space ?? this.spaceId](this);
    }

    to_css(): string {
        return super.display({ precision: 3 }).toString()
    }

    to_hex(): string {
        return this.to('srgb').toString({ format: "hex" })
    }

    norm_string(): string {
        const coords = this.get_norm().toArray().map(x => Math.round(x * 100) + "%");
        return `${this.spaceId}(${coords.join(', ')})`
    }

    is_dark(): boolean {
        // TODO: Make this better
        return this.oklab.l < 0.7;
    }

    to_vec(): Vec3 {
        return this.input.clone();
    }

    near(c: Color, eps = 1e-6, space = "jzazbz"): boolean {
        return this.distance(c, space) < eps;
    }

    toReadableString(options: ReadableStringParams = {}): string {
        const { space = "srgb", precision = 2 } = options;
        if (space === "hex") {
            return this.to("srgb").toString({ format: "hex", precision })
        }
        return this.to(space).toString({ precision })
    }
}