<script lang="ts">
    import { uniqueId } from 'lodash';
    import Copy from '$lib/assets/CopyIcon.svelte';
    import { Color } from '$lib/color';
    import CopyButton from './CopyButton.svelte';

    export let onChange: undefined | ((c: Color) => void) = undefined;
    export let color = Color.fromString('white');
    export let space = 'srgb';
    export let label = space;
    export let value = color.toReadableString({ space });

    const id = uniqueId('color_');
    let input: HTMLInputElement;

    function update_from(color: Color) {
        value = color.toReadableString({ space });
    }
    $: update_from(color);
</script>

<!-- svelte-ignore a11y-no-noninteractive-element-interactions -->
<label on:click|stopPropagation on:keypress|stopPropagation>
    <h2>{label}</h2>
    <div class="flex items-stretch gap-2 group">
        <input
            class="border-b-2 border-transparent hover:border-black bg-transparent grow"
            type="text"
            bind:this={input}
            bind:value
            on:change={(e) => {
                const val = e.currentTarget.value;
                try {
                    const c = Color.fromString(val);
                    onChange?.(c);
                } catch {
                    return;
                }
            }}
            on:blur={(e) => {
                value = color.toReadableString({ space });
            }}
        />
        <CopyButton {value} peek />
    </div>
</label>
