/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  theme: {
    extend: {
      colors: {
        quantum: {
          cyan: '#22d3ee',
          purple: '#a855f7',
          blue: '#3b82f6',
          orange: '#f97316',
          pink: '#ec4899',
          green: '#22c55e'
        }
      },
      animation: {
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
        'glow': 'glow 2s ease-in-out infinite alternate',
      },
      keyframes: {
        glow: {
          '0%': { boxShadow: '0 0 20px rgba(34, 211, 238, 0.3)' },
          '100%': { boxShadow: '0 0 40px rgba(34, 211, 238, 0.6)' },
        }
      }
    },
  },
  plugins: [],
}
