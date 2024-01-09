<script lang="ts">
    import { sx } from '$lib/classes';
    import type { Color } from '$lib/color';
    import { vec3 } from '$lib/geometry/vec';
    import ColorChip from './ColorChip.svelte';

    export let color: Color;
    export let text = false;
    export let white = color.is_dark();

    $: state = (Number(text) << 1) | Number(white);

    $: color_string = color.functional();
    $: rgb_string = color.to('srgb').toString({ precision: 2 });
    $: color_css = color.to_css();
    $: style = sx({
        ...(text
            ? {
                  bg: white ? 'white' : 'black',
                  color: color_css,
                  '--border-color': color_css
              }
            : {
                  bg: color_css,
                  color: white ? 'white' : 'black',
                  '--border-color': color_css
              }),
        columns: 'repeat( auto-fill, minmax(16rem, 1fr))'
    });
    $: white = color.is_dark();

    function increment() {
        state = (state + 1) % 4;
        white = !!(state & 1);
        text = !!(state & 2);
    }
</script>

<div
    class="grid min-h-32 gap-4 border-chip border-gray-200 box-sizing p-4 rounded-lg max-w-min"
    {style}
    on:click={(e) => {
        // Do not toggle if the user highlighted text
        // Still change on double-click to highlight
        if (window.getSelection()?.toString().trim() && e.detail <= 1) {
            return;
        }
        increment();
    }}
    on:keypress={increment}
    role="button"
    tabindex="0"
>
    <div>
        <h2>Color:</h2>
        <p>{color_string}</p>
        <h2>RGB:</h2>
        <p>{rgb_string}</p>
        {#if rgb_string !== color_css}
            <h2>CSS:</h2>
            <p>{color_css}</p>
        {/if}
    </div>
</div>

<style>
    div:hover {
        border-color: var(--border-color);
    }
</style>
