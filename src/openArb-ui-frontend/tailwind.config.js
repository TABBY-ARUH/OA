/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
    "./src/openArb-ui-frontend/**/*.{js,ts,jsx,tsx}", // 👈 add this
  ],
  theme: {
    extend: {},
  },
  plugins: [],
}
