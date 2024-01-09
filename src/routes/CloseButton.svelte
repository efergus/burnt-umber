<script lang="ts">
    import XIcon from '$lib/assets/x.svelte';
</script>

<button
    class="absolute top-0 right-0 group-hover:translate-x-0 translate-x-full transition-all duration-75 hover:stroke-3"
    on:click|stopPropagation
>
    <XIcon size={16} />
</button>
