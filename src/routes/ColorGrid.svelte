<script lang="ts">
    import { cx, sx } from '$lib/classes';
    import { AXIS } from '$lib/element/axis';
    import { ones, unit, vec3, type Vec3 } from '$lib/geometry/vec';
    import ColorChip from './ColorChip.svelte';
    import Color from 'colorjs.io';

    export let color = vec3(0, 0, 0);
    export let axis: AXIS = AXIS.Y;
    export let steps = 7;

    let colors: { color: Color; selected: boolean }[] = [];
    // export let space = 'hsv';
    function intoColor(color: Vec3) {
        return new Color('hsv', [color.x * 360, color.z * 100, color.y * 100]);
    }

    function calculateColors(color: Vec3, axis: AXIS, steps: number) {
        const unit_x = vec3(1, 0, 0);
        const unit_y = vec3(0, 1, 0);
        if (axis === AXIS.Y) {
            unit_y.copy(unit.z);
        } else if (axis === AXIS.X) {
            unit_x.copy(unit.z);
        }
        const color_z = ones.clone().sub(unit_x).sub(unit_y).multiply(color);

        const color_x = Math.max(...color.clone().multiply(unit_x));
        const color_y = Math.max(...color.clone().multiply(unit_y));
        const nx = Math.floor(color_x * steps);
        const ny = Math.floor(color_y * steps);
        const start_x = color_x - nx * (1 / steps);
        const start_y = color_y - ny * (1 / steps);
        colors = new Array(steps).fill(null).flatMap((_, i) => {
            return new Array(steps).fill(null).flatMap((_, j) => {
                return {
                    color: intoColor(
                        unit_x
                            .clone()
                            .multiplyScalar(start_x + i / steps)
                            .add(unit_y.clone().multiplyScalar(start_y + j / steps))
                            .add(color_z)
                    ),
                    selected: i === nx && j == ny
                };
            });
        });
    }

    $: calculateColors(color, axis, steps);
</script>

<div
    class="grid"
    style={sx({
        columns: steps
    })}
>
    {#each colors as color}
        <div class={color.selected ? 'border border-black' : 'border'}>
            <ColorChip color={color.color} />
        </div>
    {/each}
</div>
