<script lang="ts">
    import '../app.css';
    import Big from './Big.svelte';
    import Center from './Center.svelte';
    import Colorpicker from './Colorpicker.svelte';
</script>

<div>
    <Center>
        <Colorpicker />
    </Center>
</div>

<style>
    div {
        position: absolute;
        width: 100vw;
        height: 100vh;
        /* background-color: blueviolet; */
    }
</style>
