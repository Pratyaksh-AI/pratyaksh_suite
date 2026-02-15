/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        // Optional: We can map the specific hex codes here if we wanted to use class names like 'bg-cream'
        // But your current App.jsx uses arbitrary values (e.g. bg-[#F3F2EC]), which works automatically.
        cream: "#F3F2EC",
        neon: "#4FF978",
      },
      fontFamily: {
        sans: ['Inter', 'sans-serif'], // Or any specific font you prefer
      }
    },
  },
  plugins: [],
}