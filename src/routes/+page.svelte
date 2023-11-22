<script lang="ts">
    import '../app.css';
    // import Big from './Big.svelte';
    import Center from './Center.svelte';
    import RustColorpicker from './RustColorpicker.svelte';
    import init from '$lib/rust/rust';
    import ColorPicker from './ColorPicker.svelte';
    import Rtt from './RTT.svelte';
    import ColorAxis from './ColorAxis.svelte';
    import { AXIS } from '$lib/element/axis';
    import ColorChip from './ColorChip.svelte';
    let color = [0.5, 1, 1];

    let wasm = init();
</script>

<div id="main">
    {#await wasm}
        Loading
    {:then _}
        <Center horizontal={1}>
            <div class="flex gap-8 items-center">
                <!-- <RustColorpicker /> -->
                <Rtt />
                <div class="grid grid-cols-3">
                    <div />
                    <ColorAxis bind:color axis={AXIS.X} />
                    <ColorChip color={color} />
                    <ColorAxis bind:color axis={AXIS.Y} />
                    <ColorPicker bind:color />
                    <div />
                    <div />
                    <ColorAxis bind:color axis={AXIS.Z} />
                </div>
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
