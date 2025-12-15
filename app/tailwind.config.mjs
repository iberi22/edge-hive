/** @type {import('tailwindcss').Config} */
export default {
  content: ['./src/**/*.{astro,html,js,jsx,md,mdx,svelte,ts,tsx,vue}'],
  theme: {
    extend: {
      colors: {
        background: '#0a0a0b',
        surface: '#18181b', // zinc-900
        surfaceHighlight: '#27272a', // zinc-800
        primary: '#fbbf24', // amber-400
        primaryDim: '#d97706', // amber-600
        accent: '#8b5cf6', // violet-500
        danger: '#ef4444', // red-500
        success: '#22c55e', // green-500

        // Glassmorphism colors
        glass: {
          100: 'rgba(255, 255, 255, 0.05)',
          200: 'rgba(255, 255, 255, 0.1)',
          300: 'rgba(255, 255, 255, 0.15)',
          border: 'rgba(255, 255, 255, 0.08)',
        }
      },
      backdropBlur: {
        xs: '2px',
      },
      animation: {
        'glow': 'glow 2s ease-in-out infinite alternate',
      },
      keyframes: {
        glow: {
          '0%': { boxShadow: '0 0 5px #fbbf2433' },
          '100%': { boxShadow: '0 0 20px #fbbf2466' },
        }
      }
    },
  },
  plugins: [],
}
