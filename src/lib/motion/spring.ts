
// TODO better typing
// type SpringValue = {

// }

type SpringValue = Record<string, number>

type SpringOptions = {
    stiffness?: number,
    damping?: number,
    mass?: number,
    precision?: number,
}

export type Spring = {
    value: SpringValue,
    get(key: string): number,
    set(key: string, value: number, force?: boolean): void,
    update(): void,

    stiffness(value: number): void;
    damping(value: number): void;
}

export function spring(val: SpringValue, options: SpringOptions = {}): Spring {
    const values = { ...val };
    const target = { ...val };
    const velocity = { ...values };
    for (const key in velocity) {
        velocity[key] = 0;
    }
    const { stiffness: s = 0.4, damping: d = 0.8, precision = 0.01 } = options;
    let stiffness = Math.exp(s * 6 + 2);
    let damping = d * 2 * Math.sqrt(stiffness)
    let last = Date.now();

    return {
        value: values,
        get(key: string) {
            return values[key];
        },
        set(key: string, value: number, force = false) {
            target[key] = value;
            if (force) {
                values[key] = value;
                velocity[key] = 0;
            }
        },
        update() {
            const now = Date.now();
            const deltaT = Math.min((now - last) / 1000, 0.1);
            last = now;
            for (const key in values) {
                const force = (target[key] - values[key]) * stiffness;
                const drag = velocity[key] * damping;
                const acceleration = (force - drag);
                velocity[key] += acceleration * deltaT;
                values[key] += velocity[key] * deltaT;
                if (Math.abs(velocity[key]) < precision && Math.abs(force) < precision) {
                    velocity[key] = 0;
                    values[key] = target[key];
                }
            }
        },
        stiffness(value: number) {
            stiffness = Math.exp(value * 6 + 2);
        },
        damping(value: number) {
            damping = value * 2 * Math.sqrt(stiffness);
        }
    }
}

// Tween and tween between tweens when changed mid-tween
// export function tweeneen