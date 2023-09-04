<script lang="ts">
    import { sx } from '$lib/classes';
    import { clamp } from '$lib/math';
    import { onMount } from 'svelte';
    import Canvas from './Canvas.svelte';
    import Cursor from './Cursor.svelte';
    export let x = 0;
    export let y = 0;
    export let savedX = x;
    export let savedY = y;
    export let axes: { x?: boolean; y?: boolean } = { x: true, y: true };
    // $: wide = rect?.width > rect?.height * 2;
    // $: tall = rect?.height > rect?.width * 2;
    $: axes = {
        x: axes.x ?? false,
        y: axes.y ?? false
    };
    // $: console.log(wide, tall, axes);
    // let saved = { x, y };
    let box: HTMLDivElement;
    $: rect = box?.getBoundingClientRect();

    const pos = (clientX: number, clientY: number) => {
        x = clamp((clientX - rect.x) / rect.width, 0, 1);
        y = clamp((rect.y - clientY) / rect.height + 1, 0, 1);
        return { x, y };
    };

    const save = (e: MouseEvent) => {
        let p = pos(e.clientX, e.clientY);
        savedX = p.x;
        savedY = p.y;
    };

    const onmouse = (e: MouseEvent) => {
        pos(e.clientX, e.clientY);
        if (e.buttons & 1) save(e);
    };

    const revert = () => {
        x = savedX;
        y = savedY;
    };
    
    const style = sx({ bg: 0.5, w: '100%', h: '100%', pos: 'relative' });
</script>

<div
    {style}
    role="slider"
    tabindex="0"
    aria-valuenow={x}
    bind:this={box}
    on:mousedown={save}
    on:mouseup={save}
    on:mousemove={onmouse}
    on:mouseleave={(e) => {
        onmouse(e);
        revert();
    }}
    on:blur={revert}
>
    <Canvas/>
    <Cursor color={[1, 0, 1]} {axes} x={axes.x ? x * (rect?.width ?? 0) : 0} y={axes.y ? y * (rect?.height ?? 0) : 0}/>
    <Cursor color={[0, 1, 0]} {axes} x={axes.x ? savedX * (rect?.width ?? 0) : 0} y={axes.y ? savedY * (rect?.height ?? 0) : 0}/>
</div>
