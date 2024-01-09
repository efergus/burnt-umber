import { persisted } from 'svelte-persisted-store'

export const selected_colors = persisted<string[]>('selected_colors', []);
