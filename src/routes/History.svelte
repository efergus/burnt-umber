<script lang="ts">
    import { cx, sx } from '$lib/classes';
    import { Color } from '$lib/color';
    import type { Vec3 } from '$lib/geometry/vec';
    import ColorChip from './ColorChip.svelte';
    import XIcon from '$lib/assets/x.svelte';
    import CloseButton from './CloseButton.svelte';
    import CopyButton from './CopyButton.svelte';

    export let color: Vec3;
    export let saved_color: Vec3;
    export let colors: Color[];

    export let onClick: undefined | ((c: Color) => void) = undefined;

    if (typeof window !== 'undefined') {
        colors = JSON.parse(window.localStorage?.getItem('selected_colors') ?? '[]').map(
            (s: string) => Color.fromString(s)
        );
        if (colors.length) {
            color = colors[colors.length - 1].get_norm();
            saved_color = color.clone();
        }
    }

    function update_selected_colors() {
        const value = JSON.stringify(colors.map((c) => c.toString()));
        window.localStorage.setItem(
            'selected_colors',
            JSON.stringify(colors.map((c) => c.toString()))
        );
    }
    function push(c: Color) {
        colors.push(c);
        colors = colors;
        update_selected_colors();
    }
    function remove(i: number, count = 1) {
        colors.splice(i, count);
        colors = colors;
        update_selected_colors();
    }
    export const select = (c: Color, save = true) => {
        const l = colors.length;
        if (!l) {
            return push(c);
        }
        const current = colors[l - 1];
        if (save && current.near(c) && !colors[l - 2]?.near(c)) {
            return push(c);
        }
        colors[l - 1] = c;
        update_selected_colors();
    };
</script>

<div class="sticky top-0 bg-slate-100 px-8 z-50">
    <CloseButton
        positioning="top-2 -left-2"
        on:click={() => {
            remove(0, colors.length - 1);
        }}
    />
    <div
        class="grid min-h-chip overflow-x-auto"
        style={sx({
            columns: 'repeat( auto-fill, minmax(6rem, 1fr))'
        })}
    >
        {#each colors as color, index}
            <ColorChip
                classes="first:rounded-l last:rounded-r"
                {color}
                selected={index === colors.length - 1}
                {onClick}
            >
                <div
                    class={cx(
                        'group relative w-full h-full overflow-hidden flex flex-col justify-end items-start px-1',
                        color.is_dark() && 'text-white'
                    )}
                >
                    <CloseButton on:click={() => remove(index)} />
                    <div
                        class="absolute top-0 left-0 pl-0.5 group-hover:translate-x-0 -translate-x-full transition-all duration-75 hover:stroke-3"
                    >
                        <CopyButton value={color.to_hex()} size={16} />
                    </div>
                    {color.to_hex()}
                </div>
            </ColorChip>
        {/each}
    </div>
</div>
