/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "all",
  content: [
    // include all rust, html and css files in the src directory
    "./src/**/*.{rs,html,css}",
    // include all html files in the output (dist) directory
    "../dist/**/*.html",
  ],
  theme: {
    minWidth: {
      88: "22rem",
    },
    minHeight: {
      16: "4rem",
    },
    extend: {},
  },
  plugins: [],
};
