<script lang="ts">
    export let wide = true;
    export let tall = true;

    $: width = wide ? '100%' : '0px';
    $: height = tall ? '100%' : '0px';
</script>

<div style:width style:height>
    <slot />
</div>

<style>
    div {
        background-color: brown;
    }
</style>
