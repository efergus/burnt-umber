<script lang="ts">
    import { greet } from "$lib/rust/pkg/rust";

    export let horizontal: -1 | 0 | 1 = 0;
    export let vertical: -1 | 0 | 1 = 0;

    $: justify = ['start', 'center', 'end'][horizontal + 1];
    $: items = ['start', 'center', 'end'][vertical + 1];
    greet("Wasm in Svelte");
</script>

<div style:justify-content={justify} style:align-items={items}>
    <slot />
</div>

<style>
    div {
        display: flex;
        width: 100%;
        height: 100%;
    }
</style>
