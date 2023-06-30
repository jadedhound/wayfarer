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
      'sans': ['Open Sans','ui-sans-serif'],
      'sans-condensed': ['Open Sans Condensed','ui-sans-serif'],
      'serif': ['EB Garamond', 'ui-serif'],
      'mono': ['ui-monospace'],
    },
    extend: {
      colors: {
        wfbutton: '#450a0a',
        wfborder: '#d4d4d8',
        wfbrown: '#92400e',
        wfblue: '#0369a1'
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
