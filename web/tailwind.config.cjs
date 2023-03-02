/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      colors:{ 
         // Dark theme elevation levels and text emphasis as recommended by material.io https://material.io/design/color/dark-theme.html#properties
         elevation: {
          0: "#121212",
          1: "#1d1d1d",
          2: "#212121",
          3: "#242424",
          4: "#272727",
          6: "#2c2c2c",
          8: "#2d2d2d",
          12: "#323232",
          16: "#353535",
          24: "#373737",
      },
        emphasis: {
            high: "#e0e0e0",
            medium: "#a0a0a0",
            low: "#6c6c6c"
        },
      }
    },
  },
  plugins: [],
}
