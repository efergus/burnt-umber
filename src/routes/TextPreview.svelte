<script lang="ts">
    import ChevronDown from '$lib/assets/chevron-down.svelte';
    import ChevronRight from '$lib/assets/chevron-right.svelte';
    import { cx, sx } from '$lib/classes';
    import type { Color } from '$lib/color';
    import { vec3 } from '$lib/geometry/vec';
    import { space } from 'postcss/lib/list';
    import ColorChip from './ColorChip.svelte';
    import ColorTextInput from './ColorTextInput.svelte';
    import CopyButton from './CopyButton.svelte';

    export let onChange: undefined | ((c: Color) => void) = undefined;
    export let color: Color;
    export let text = false;
    export let white = color.is_dark();
    export let detail = false;

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
    class="grid min-h-32 relative border-chip border-gray-200 box-sizing p-4 rounded-lg max-w-min"
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
    <button
        class={cx(
            'absolute right-1 top-2 stroke-2 hover:stroke-3 transition-transform',
            detail && 'rotate-180'
        )}
        on:click|stopPropagation={() => {
            detail = !detail;
            console.log(detail);
        }}
    >
        <ChevronDown />
    </button>
    <div class="grid">
        <div>
            <div class="flex gap-2">
                <CopyButton value={color.to_hex()} />
                <h2>Color:</h2>
            </div>
            <p>{color_string}</p>
        </div>
        <ColorTextInput space="sRGB" {color} {onChange} />
        {#if rgb_string !== color_css}
            <div>
                <h2>CSS:</h2>
                <div class="flex gap-2">
                    <!-- <CopyButton value={color_css} /> -->
                    <p>{color_css}</p>
                </div>
            </div>
        {/if}
    </div>
    {#if detail}
        <ColorTextInput {color} space={'hex'} {onChange} />
        <ColorTextInput {color} space={'OKLab'} {onChange} />
    {/if}
</div>

<style>
    div:hover {
        border-color: var(--border-color);
    }
</style>
