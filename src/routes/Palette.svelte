
<script lang="ts">
    import { sx } from "$lib/classes";

    export let colors: string[] = [];
</script>

<div class="flex flex-col">
    <div class="flex flex-col">
        {#each colors as color}
            <div class="h-16 w-full" style={sx({
                bg: color
            })}></div>
        {/each}
    </div>
</div>