/** @type {import('tailwindcss').Config} */
module.exports = {
  content: [
    './pages/**/*.{js,ts,jsx,tsx,mdx}',
    './components/**/*.{js,ts,jsx,tsx,mdx}',
    './app/**/*.{js,ts,jsx,tsx,mdx}',
  ],
  theme: {
    extend: {
      colors: {
        'input-bg': '#1A1A1A',
      },
      backgroundImage: {
        'gradient-radial': 'radial-gradient(circle at center, var(--tw-gradient-stops))',
      },
      animation: {
        'float-boats': 'float-boats 80s linear infinite',
        'float-horizontal': 'float-horizontal 100s linear infinite',
        'fadeIn': 'fadeIn 1.5s ease-out',
        'slideUp': 'slideUp 1.5s ease-out',
      },
      keyframes: {
        'float-boats': {
          '0%': { backgroundPosition: '0% 0%' },
          '100%': { backgroundPosition: '200% 200%' }
        },
        'float-horizontal': {
          '0%': { backgroundPosition: '0% 50%' },
          '100%': { backgroundPosition: '200% 50%' }
        },
        fadeIn: {
          '0%': { opacity: '0' },
          '100%': { opacity: '1' },
        },
        slideUp: {
          '0%': { transform: 'translateY(20px)', opacity: '0' },
          '100%': { transform: 'translateY(0)', opacity: '1' },
        }
      },
      backdropBlur: {
        'center': 'blur(8px)',
      },
      backgroundSize: {
        'boat-pattern': '60px 60px',
      },
    },
  },
  darkMode: 'class',
  plugins: [],
}
