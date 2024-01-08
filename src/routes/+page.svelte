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
    import { vec3, type Vec3 } from '$lib/geometry/vec';
    import ColorGrid from './ColorGrid.svelte';
    import { Color } from '$lib/color';
    import History from './History.svelte';
    let color = vec3(0.5, 1, 1);
    let saved_color = vec3(0.5, 1, 1);
    let history: History;

    function set_color(c: Color) {
        color = c.get_norm();
        saved_color = c.get_norm();
        history.select(c);
    }

    function intoHsvColor(color: Vec3) {
        return new Color('hsv', color);
    }

    $: selected_color = intoHsvColor(color);
</script>

<div id="main">
    <!-- {#await wasm}
        Loading
    {:then _} -->
    <History bind:this={history} />
    <Center>
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
                <ColorChip color={selected_color} />
                <ColorAxis bind:color bind:saved_color axis={AXIS.Y} />
                <ColorPicker bind:color bind:saved_color />
                <Palette color={selected_color} onClick={set_color} />
                <div />
                <ColorAxis bind:color bind:saved_color axis={AXIS.Z} />
            </div>
            <div>
                <ColorGrid bind:color onClick={set_color} />
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
