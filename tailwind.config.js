/** @type {import('tailwindcss').Config} */
export default {
    content: ['./src/**/*.{html,js,svelte,ts}'],
    theme: {
        extend: {
            minWidth: {
                'chip': '4rem'
            },
            minHeight: {
                'chip': '4rem'
            },
        }
    },
    plugins: []
};
