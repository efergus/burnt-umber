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
    import { near, vec3, type Vec3 } from '$lib/geometry/vec';
    import ColorGrid from './ColorGrid.svelte';
    import { Color } from '$lib/color';
    import History from './History.svelte';
    import type { CursorSpec } from '$lib/element/cursor';
    import TextPreview from './TextPreview.svelte';
    import ColorBunch from './ColorBunch.svelte';
    let color = vec3(0.5, 1, 1);
    let saved_color = vec3(0.5, 1, 1);
    let space_clicked_color = vec3(0.5, 1, 1);
    let history: History;
    let history_colors: Color[] = [];

    function set_color(c: Color, save = true) {
        color = c.get_norm('hsv');
        history?.select(c, save);
        if (save) {
            saved_color = c.get_norm();
        }
    }

    function intoHsvColor(color: Vec3) {
        return new Color('hsv', color);
    }

    function doubleClickColor(c: Color) {
        set_color(c, near(c.get_norm(), space_clicked_color));
        space_clicked_color = c.get_norm();
    }

    $: selected_color = intoHsvColor(color);
    $: set_color(selected_color, false);
    $: cursors = [
        ...history_colors.map<CursorSpec>((c, i) => ({
            pos: c.get_norm(),
            size: ((i + 1) / history_colors.length) * 0.95 + 0.05
        }))
    ];
</script>

<div id="main" class="flex flex-col gap-4 justify-stretch relative stroke-2">
    <!-- {#await wasm}
        Loading
    {:then _} -->
    <History
        bind:this={history}
        bind:color
        bind:saved_color
        bind:colors={history_colors}
        onClick={set_color}
    />
    <div class="px-8">
        <TextPreview color={selected_color} onChange={(c) => set_color(c, true)} />
    </div>
    <div class="grow">
        <Center>
            <div class="flex gap-8 flex-wrap">
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
                    <ColorAxis
                        bind:color
                        bind:saved_color
                        axis={AXIS.X}
                        {cursors}
                        onClick={doubleClickColor}
                    />
                    <ColorChip color={selected_color} onClick={set_color} />
                    <ColorAxis
                        bind:color
                        bind:saved_color
                        axis={AXIS.Y}
                        {cursors}
                        onClick={doubleClickColor}
                    />
                    <ColorPicker bind:color bind:saved_color {cursors} onClick={doubleClickColor} />
                    <Palette color={selected_color} onClick={set_color} />
                    <div />
                    <ColorAxis
                        bind:color
                        bind:saved_color
                        axis={AXIS.Z}
                        {cursors}
                        onClick={doubleClickColor}
                    />
                </div>
                <div>
                    <ColorGrid bind:color onClick={set_color} />
                </div>
                <!-- <div>
                    <ColorGrid bind:color onClick={set_color} axis={AXIS.Z} />
                </div> -->
                <ColorBunch colors={history_colors} />
            </div>
        </Center>
    </div>
    <!-- {/await} -->
</div>

<style>
    div#main {
        position: absolute;
        width: 100vw;
        height: 100vh;
    }
</style>
