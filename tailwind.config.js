/** @type {import('tailwindcss').Config} */
export default {
  content: ["./src/**/*.{html,js,svelte,ts}"],
  theme: {
    extend: {
      colors: {
        primary: {
          500: "#2563eb",
          600: "#1d4ed8",
          700: "#1e40af",
        },
        success: {
          500: "#10b981",
          600: "#059669",
        },
        water: {
          blue: "#3b82f6",
        },
      },
      fontFamily: {
        sans: ["Inter", "Avenir", "Helvetica", "Arial", "sans-serif"],
      },
      fontSize: {
        stat: "2rem",
      },
      spacing: {
        18: "4.5rem",
      },
    },
  },
  plugins: [],
};
