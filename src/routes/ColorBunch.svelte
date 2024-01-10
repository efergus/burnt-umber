<script lang="ts">
    import { cx, sx } from '$lib/classes';
    import type { Color } from '$lib/color';

    export let colors: Color[] = [];
</script>

<div class="flex flex-col flex-wrap">
    {#each colors as color}
        <p
            class={cx('py-1 px-2 first:rounded-t last:rounded-b', color.is_dark() && 'text-white')}
            style={sx({
                bg: color.to_css()
            })}
        >
            {color.to_hex()}
        </p>
    {/each}
</div>
