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

    // const id = uniqueId('text_');

    function update_from(color: Color) {
        value = color.toReadableString({ space });
    }
    $: update_from(color);
</script>

<label
    on:click|stopPropagation
    on:keypress={(e) => {
        e.stopPropagation();
    }}
>
    <h2>{label}</h2>
    <div class="flex items-stretch gap-2">
        <!-- <CopyButton value={color.to('srgb').toString({ format: 'hex', precision: 2 })} /> -->
        <input
            class="border-b-2 border-transparent hover:border-black bg-transparent grow"
            type="text"
            bind:this={input}
            bind:value
            on:change={(e) => {
                const val = e.currentTarget.value;
                try {
                    // console.log('bbbbb', c.toReadableString());
                    const c = Color.fromString(val);
                    console.log('ccccc', c.toReadableString());
                    // color = c;
                    onChange?.(c);
                    // input?.blur();
                } catch {
                    return;
                }
            }}
            on:blur={(e) => {
                // console.log('aaaa', c.toReadableString());
                value = color.toReadableString({ space });
            }}
        />
    </div>
</label>
