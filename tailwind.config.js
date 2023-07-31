/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "./static/pages/*.html",
    "index.html",
    "tailwind_stub.css"
  ],
  theme: {
    fontFamily: {
      'sans': ['Open Sans Condensed', 'Open Sans','ui-sans-serif'],
      'serif': ['EB Garamond', 'ui-serif'],
      'mono': ['ui-monospace'],
    },
    extend: {
      animation: {
        "fade": "fadeIn .5s ease-in",
        "popin": "popIn .3s ease-out"
      },
      keyframes: {
        fadeIn: {
          '0%': {
            opacity: '.2',
            transform: 'translateY(2%)'
          },
        },
        popIn: {
          '0%': {
            opacity: '.2',
            transform: 'scale(0.8)',
          },
          '100%': {
            opacity: '1',
            transform: 'scale(1)',
          },
        },
      }
    }
  },
  plugins: [],
}
