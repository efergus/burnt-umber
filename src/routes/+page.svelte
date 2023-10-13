<script lang="ts">
    import '../app.css';
    import Canvas from './Canvas.svelte';
    // import Big from './Big.svelte';
    import Center from './Center.svelte';
    import Colorpicker from './Colorpicker.svelte';
    import init from '$lib/rust/rust';

    let wasm = init();
</script>

<div id="main">
    {#await wasm}
        Loading
    {:then _}
        <Center horizontal={1}>
            <Colorpicker />
        </Center>
    {/await}
</div>

<style>
    div#main {
        position: absolute;
        width: 100vw;
        height: 100vh;
    }
</style>
