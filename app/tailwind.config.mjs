/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
  theme: {
    extend: {
      colors: {
        background: '#020617', // slate-950
        surface: '#1e293b', // slate-800
        surfaceHighlight: '#334155', // slate-700
        primary: '#f97316', // orange-500
        primaryDim: '#ea580c', // orange-600
        secondary: '#0ea5e9', // sky-500
        accent: '#0ea5e9', // sky-500 (mapped to secondary for now)
        danger: '#ef4444', // red-500
        success: '#10b981', // emerald-500
        muted: '#64748b', // slate-500
        textBody: '#cbd5e1', // slate-300
        textHeading: '#ffffff',

        // Glassmorphism colors
        glass: {
          100: 'rgba(30, 41, 59, 0.4)', // slate-800 with opacity
          200: 'rgba(30, 41, 59, 0.6)',
          300: 'rgba(30, 41, 59, 0.8)',
          border: 'rgba(255, 255, 255, 0.1)',
        }
      },
      fontFamily: {
        sans: ['Inter', 'sans-serif'],
        mono: ['Fira Code', 'monospace'],
      },
      backdropBlur: {
        xs: '2px',
      },
      animation: {
        'glow': 'glow 2s ease-in-out infinite alternate',
        'pulse-slow': 'pulse 3s cubic-bezier(0.4, 0, 0.6, 1) infinite',
      },
      keyframes: {
        glow: {
          '0%': { boxShadow: '0 0 5px #f9731633' },
          '100%': { boxShadow: '0 0 20px #f9731666' },
        }
      }
    },
  },
  plugins: [],
}
