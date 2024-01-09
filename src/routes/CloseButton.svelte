<script lang="ts">
    import XIcon from '$lib/assets/x.svelte';
</script>

<button
    class="absolute top-0 right-0 pl-0.5 pb-0.5 group-hover:translate-x-0 translate-x-full rounded-bl-lg transition-all duration-75 stroke-2 hover:stroke-[3px]"
    on:click|stopPropagation
>
    <XIcon size={16} />
</button>
