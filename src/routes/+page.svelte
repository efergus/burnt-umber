<script lang="ts">
    import '../app.css';
    // import Big from './Big.svelte';
    import Center from './Center.svelte';
    import RustColorpicker from './RustColorpicker.svelte';
    import init from '$lib/rust/rust';
    import ColorPicker from './ColorPicker.svelte';

    let wasm = init();
</script>

<div id="main">
    {#await wasm}
        Loading
    {:then _}
        <Center horizontal={1}>
            <div class="flex gap-8 items-center">
                <!-- <RustColorpicker /> -->
                <ColorPicker />
            </div>
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
