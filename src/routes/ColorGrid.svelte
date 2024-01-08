<script lang="ts">
    import { cx, sx } from '$lib/classes';
    import { AXIS } from '$lib/element/axis';
    import { unit, vec3, type Vec3 } from '$lib/geometry/vec';
    import ColorChip from './ColorChip.svelte';
    import { Color } from '$lib/color';

    export let color = vec3(0, 0, 0);
    export let axis: AXIS = AXIS.Y;
    export let steps = 7;
    export let scale_x = 0.1;
    export let scale_y = 1;
    export let onClick: undefined | ((c: Color, save?: boolean) => void) = undefined;

    let colors: { color: Color; selected: boolean }[] = [];
    function intoColor(color: Vec3) {
        return new Color('hsv', color);
    }

    function calculateColors(
        color: Vec3,
        axis: AXIS,
        steps: number,
        scale_x: number,
        scale_y: number
    ) {
        const unit_x = vec3(1, 0, 0);
        const unit_y = vec3(0, 1, 0);
        if (axis === AXIS.Y) {
            unit_y.copy(unit.z);
        } else if (axis === AXIS.X) {
            unit_x.copy(unit.z);
        }

        const center = (steps - 1) / 2;
        colors = new Array(steps).fill(null).flatMap((_, i) => {
            return new Array(steps).fill(null).flatMap((_, j) => {
                return {
                    color: intoColor(
                        color
                            .clone()
                            .addScaledVector(unit_x, ((j - center) / steps) * scale_x)
                            .addScaledVector(unit_y, ((center - i) / steps) * scale_y)
                    ),
                    selected: i === center && j === center
                };
            });
        });
    }

    $: calculateColors(color, axis, steps, scale_x, scale_y);
</script>

<div
    class="grid"
    style={sx({
        columns: '1fr min-content'
    })}
>
    <div
        class="grid"
        style={sx({
            columns: steps
        })}
    >
        {#each colors as color}
            <div>
                <ColorChip
                    color={color.color}
                    selected={color.selected}
                    onClick={(c) => onClick?.(c, color.selected)}
                />
            </div>
        {/each}
    </div>
    <input
        type="range"
        style={sx({
            appearance: 'slider-vertical',
            writing_mode: 'vertical-lr',
            width: '2rem'
        })}
        min={0.1}
        max={1}
        step={0.05}
        value={scale_y}
        on:input={(e) => {
            const value = e.currentTarget.valueAsNumber;
            scale_y = value;
        }}
    />
    <input
        type="range"
        style={sx({
            height: '2rem'
        })}
        min={0.1}
        max={1}
        step={0.05}
        value={scale_x}
        on:input={(e) => {
            const value = e.currentTarget.valueAsNumber;
            scale_x = value;
        }}
    />
</div>
