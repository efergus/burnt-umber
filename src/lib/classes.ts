type val = string | number | (string | number)[];

export const cx = (...classes: (string | false | undefined | null)[]) =>
    classes.filter((x) => x).join(' ');

const isNum = (x: val): x is number => typeof x === "number"

const sx_map: Record<string, val | ((v: val, k: string) => string | string[])> = {
    w: 'width',
    h: 'height',
    x: 'left',
    y: 'top',
    z: 'z-index',
    bg: (v: val) => [`background-color: ${to_color(v)}`],
    p: 'padding',
    px: (v: val) => [`padding-left: ${to_px(v)}`, `padding-right: ${to_px(v)}`],
    py: (v: val) => [`padding-top: ${to_px(v)}`, `padding-bottom: ${to_px(v)}`],
    pos: 'position',
    columns: (v: val) => {
        return isNum(v) ? `grid-template-columns: repeat(${v}, 1fr)` : `grid-template-columns: ${v}`
    },
    rows: (v: val) => {
        return isNum(v) ? `grid-template-rows: repeat(${v}, 1fr)` : `grid-template-rows: ${v}`
    },
};

const pad_array = <T>(arr: T[], len: number, fill: T) => {
    return arr.length > len ? arr : arr.concat(Array(len - arr.length).fill(fill));
};

const to_color_arr = (v: val): string[] =>
    Array.isArray(v)
        ? pad_array(v, 4, 1).map((x, i) =>
            (i < 3 && !(typeof x === 'string') ? x * 255 : x).toString()
        )
        : to_color_arr([v, v, v, 1]);

const to_color = (v: val): string =>
    typeof v === 'string' ? v : `rgba(${to_color_arr(v).join(', ')})`;

const to_px = (v: val) => `${v}${typeof v === 'number' ? 'px' : ''}`;

const sx_mapper = (k: string, v: val) => {
    const fn = sx_map[k];
    if (typeof fn === 'function') {
        const res = fn(v, k);
        return Array.isArray(res) ? res.join('; ') : res;
    }
    return `${fn ?? k.replace(/_/g, '-')}: ${to_px(v)}`;
};

export const sx = (obj: Record<string, val>) =>
    Object.entries(obj)
        .map(([k, v]) => sx_mapper(k, v))
        .join('; ');

// Create classes and styles
// export const scx =
