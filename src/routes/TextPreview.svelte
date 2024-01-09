<script lang="ts">
    import { sx } from '$lib/classes';
    import type { Color } from '$lib/color';
    import { vec3 } from '$lib/geometry/vec';
    import ColorChip from './ColorChip.svelte';

    export let color: Color;
    export let text = false;
    export let white = false;

    // style={sx({
    //     columns: 'repeat( auto-fill, minmax(64px, 1fr))'
    // })}
    $: color_string = color.functional();
    $: rgb_string = color.to('srgb').toString({ precision: 2 });
    $: color_css = color.to_css();
    $: style = sx(
        text
            ? {
                  bg: white ? 'white' : 'black',
                  color: color_css,
                  '--border-color': color_css
              }
            : {
                  bg: color_css,
                  color: white ? 'white' : 'black',
                  '--border-color': color_css
              }
    );

    $: white = color.is_dark();
</script>

<div
    class="border-chip border-gray-200 box-sizing p-4 rounded-lg"
    {style}
    on:click={() => {
        if (window.getSelection()?.toString().trim()) {
            return; // Do not toggle if the user highlighted text
        }
        white = !white;
    }}
    on:keypress={() => {
        white = !white;
    }}
    role="button"
    tabindex="0"
>
    <h2>Color:</h2>
    <p>{color_string}</p>
    <h2>RGB:</h2>
    <p>{rgb_string}</p>
    {#if rgb_string !== color_css}
        <h2>CSS:</h2>
        <p>{color_css}</p>
    {/if}
</div>

<style>
    div:hover {
        border-color: var(--border-color);
    }
</style>
