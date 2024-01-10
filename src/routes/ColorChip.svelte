<script lang="ts">
    import { cx, sx } from '$lib/classes';
    import { Color } from '$lib/color';
    import { vec3, type Vec3 } from '$lib/geometry/vec';

    export let selected = false;
    export let color = new Color('srgb', vec3(1, 1, 1));
    export let onClick: undefined | ((c: Color) => void) = undefined;
    export let onDoubleClick: undefined | ((c: Color) => void) = undefined;
    export let rounded = false;
    export let classes = '';

    let clicked_color: undefined | Color = undefined;

    $: css = color.to_css();
</script>

<button
    class={cx(
        'relative w-full h-full min-h-chip min-w-chip border-box border-chip hover:!border-white peer-hover:!border-white',
        rounded && 'rounded',
        classes
    )}
    style={sx({
        bg: css,
        border_color: selected ? 'black' : css
    })}
    on:click={(event) => {
        if (event.detail === 1) {
            clicked_color = color;
        } else if (clicked_color) {
            onDoubleClick?.(clicked_color);
            clicked_color = undefined;
        }
        onClick?.(color);
    }}
>
    <slot />
</button>
