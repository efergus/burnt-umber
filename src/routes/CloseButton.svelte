<script lang="ts">
    import XIcon from '$lib/assets/x.svelte';

    export let positioning = 'top-0 right-0 group-hover:translate-x-0';
</script>

<button
    class={`absolute ${positioning} translate-x-full transition-all duration-75 hover:stroke-3`}
    on:click|stopPropagation
>
    <XIcon size={16} />
</button>
