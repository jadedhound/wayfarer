/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    "./src/**/*.rs",
    "index.html",
    "tailwind_stub.css"
  ],
  theme: {
    fontFamily: {
      'sans': ['Open Sans','ui-sans-serif'],
      'sans-condensed': ['Open Sans Condensed','ui-sans-serif'],
      'serif': ['EB Garamond', 'ui-serif'],
      'mono': ['ui-monospace'],
    },
    extend: {
      spacing: {
        '128': '32rem',
      },
      animation: {
        "fade": "fadeIn .5s ease-in",
      },
      keyframes: {
        fadeIn: {
          '0%': {
            opacity: '.2',
            transform: 'translateY(2%)'
          },
        },
      }
    }
  },
  plugins: [],
}
