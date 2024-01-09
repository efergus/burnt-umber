<script lang="ts">
    import CopyIcon from '$lib/assets/CopyIcon.svelte';
    import { cx, sx } from '$lib/classes';

    export let value = '';
    export let timeout = 1000;
    export let size = 24;
    export let animate = false;
    export let peek = false;

    let text: HTMLElement;
    let width = 0;
</script>

<button
    class={cx("hover:stroke-3 active:stroke-2 relative", peek && "invisible group-hover:visible")}
    on:click|stopPropagation={() => {
        if (!value) return;
        navigator.clipboard.writeText(value);
        console.log('Copied', value);
        width = 220;
        setTimeout(() => (width = 0), timeout);
    }}
>
    {#if animate}
        <div class="absolute w-full h-full z-50 translate-x-full">
            <div
                class="overflow-hidden transition-all rounded-full flex justify-center"
                style={sx({ width })}
            >
                <div class="whitespace-nowrap bg-white text-black px-2 w-full">
                    Text Copied to Clipboard
                </div>
            </div>
        </div>
    {/if}
    <CopyIcon {size} /></button
>
