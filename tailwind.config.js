/** @type {import('tailwindcss').Config} */
module.exports = {
  theme: {
    extend: {
      screens: {
        "3xl": "1920px",
        "4xl": "2560px",
        "5xl": "3840px",
        "6xl": "5120px",
        "7xl": "8640px",
      },
    },
  },
  content: {
    files: ["*.html", "./src/**/*.rs"],
  },
  plugins: [
    require("@tailwindcss/typography"),
    require("daisyui"),
  ],
  daisyui: {
    themes: ["retro", "light", "dark"],
  },
  darkMode: ['class', '[data-theme="dark"]']
};
