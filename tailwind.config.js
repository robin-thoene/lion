/** @type {import('tailwindcss').Config} */
module.exports = {
  content: ["*.html", "./src/**/*.rs"],
  theme: {
    fontFamily: {
      "jetbrains-mono": ['"JetBrains Mono"'],
    },
    extend: {
      colors: {
        primary: "#bd93f9",
        "primary-alt": "#7d4aff",
      },
    },
  },
  plugins: [],
};
