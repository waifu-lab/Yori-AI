import { addDynamicIconSelectors } from "@iconify/tailwind";

/** @type {import('tailwindcss').Config} */
module.exports = {
    contents: [
        './src/**/*.{html, svelte, js, ts}'
    ],
    theme: {
        extend: {

        }
    },
    plugins: [addDynamicIconSelectors()]
}