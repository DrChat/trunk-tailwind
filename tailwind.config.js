/** @type {import('tailwindcss').Config} */
module.exports = {
  mode: "jit",
  content: ["src/**/*.rs", "index.html"],
  theme: {
    extend: {},
  },
  plugins: [require("daisyui")],
}

