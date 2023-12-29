<script lang="ts">
    import '../app.css';
    // import Big from './Big.svelte';
    import Center from './Center.svelte';
    // import RustColorpicker from './RustColorpicker.svelte';
    // import init from '$lib/rust/rust';
    import ColorPicker from './ColorPicker.svelte';
    import Rtt from './RTT.svelte';
    import ColorAxis from './ColorAxis.svelte';
    import { AXIS } from '$lib/element/axis';
    import ColorChip from './ColorChip.svelte';
    import Palette from './Palette.svelte';
    import { sx } from '$lib/classes';
    import { vec3 } from '$lib/geometry/vec';
    let color = vec3(0.5, 1, 1);
    let saved_color = vec3(0.5, 1, 1);
    let input = vec3(0, 0, 0);

    // let wasm = init();
    let palette_colors: string[] = [];
    $: palette_colors = new Array(6)
        .fill('')
        .map(
            (_, i) =>
                `hsl(${(color.x + i / 12) * 360}, ${(color.z - i / 12) * 100}%, ${
                    (color.y - i / 12) * 100
                }%)`
        );
</script>

<div id="main">
    <!-- {#await wasm}
        Loading
    {:then _} -->
    <Center horizontal={1}>
        <div class="flex gap-8 items-center">
            <!-- <RustColorpicker /> -->
            <!-- <Rtt /> -->
            <div
                class="grid"
                style={sx({
                    grid_template_columns: '4rem 1fr 4rem',
                    grid_template_rows: '4rem 1fr 4rem'
                })}
            >
                <div />
                <ColorAxis bind:color bind:saved_color axis={AXIS.X} />
                <ColorChip {color} />
                <ColorAxis bind:color bind:saved_color axis={AXIS.Y} />
                <ColorPicker bind:color bind:saved_color />
                <Palette colors={palette_colors} />
                <div />
                <ColorAxis bind:color bind:saved_color axis={AXIS.Z} />
            </div>
        </div>
    </Center>
    <!-- {/await} -->
</div>

<style>
    div#main {
        position: absolute;
        width: 100vw;
        height: 100vh;
    }
</style>
