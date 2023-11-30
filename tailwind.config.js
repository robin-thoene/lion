/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    extend: {
      colors: {
        primary: "#bd93f9",
      },
    },
  },
  plugins: [],
};
