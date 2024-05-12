module.exports = {
  content: ["./src/**/*.rs", "./index.html", "./src/**/*.html", "./src/**/*.css"],
  theme: {
    screens: {
      'sm': '640px',
      'md': '768px',
      'lg': '1024px',
      'xl': '1280px',
      '2xl': '1536px',
    },
    extend: {
      fontFamily: {
        oswald: ["Oswald", "sans-serif"],
      },
    },
  },
  variants: {},
  plugins: [],
};

