/** @type {import('tailwindcss').Config} */
export default {
  content: [
    "./index.html",
    "./src/**/*.{js,ts,jsx,tsx}",
  ],
  darkMode: 'class',
  theme: {
    extend: {
      colors: {
        // Dark Theme (Default)
        background: {
          primary: '#131722',
          secondary: '#1E222D',
          tertiary: '#2A2E39'
        },
        text: {
          primary: '#D1D4DC',
          secondary: '#B2B5BE',
          tertiary: '#787B86'
        },
        accent: {
          DEFAULT: '#2962FF',
          primary: '#2962FF',
          hover: '#1E53E5',
          light: 'rgba(41, 98, 255, 0.1)'
        },
        success: {
          DEFAULT: '#26A69A',
          light: 'rgba(38, 166, 154, 0.1)'
        },
        danger: {
          DEFAULT: '#EF5350',
          light: 'rgba(239, 83, 80, 0.1)'
        },
        warning: {
          DEFAULT: '#FF9800',
          light: 'rgba(255, 152, 0, 0.1)'
        },
        info: {
          DEFAULT: '#2196F3',
          light: 'rgba(33, 150, 243, 0.1)'
        },
        border: {
          primary: '#2A2E39',
          secondary: '#363A45',
          hover: '#434651'
        }
      },
      ringColor: {
        DEFAULT: '#2962FF',
        accent: '#2962FF',
        danger: '#EF5350',
      },
      fontFamily: {
        sans: ['Inter', 'system-ui', '-apple-system', 'sans-serif'],
        mono: ['JetBrains Mono', 'monospace']
      },
      fontSize: {
        '2xs': '11px',
        'xs': '12px',
        'sm': '14px',
        'base': '14px',
        'lg': '16px',
        'xl': '20px',
        '2xl': '24px',
        '3xl': '32px'
      },
      spacing: {
        '1': '4px',
        '2': '8px',
        '3': '12px',
        '4': '16px',
        '5': '20px',
        '6': '24px',
        '8': '32px',
        '10': '40px',
        '12': '48px'
      },
      screens: {
        'sm': '640px',
        'md': '768px',
        'lg': '1024px',
        'xl': '1280px',
        '2xl': '1536px'
      },
      animation: {
        'skeleton-pulse': 'skeleton-pulse 1.5s ease-in-out infinite',
        'price-flash-up': 'price-flash-up 0.5s ease-out',
        'price-flash-down': 'price-flash-down 0.5s ease-out',
        'toast-slide-in': 'toast-slide-in 0.3s ease-out'
      },
      keyframes: {
        'skeleton-pulse': {
          '0%, 100%': { opacity: '1' },
          '50%': { opacity: '0.5' }
        },
        'price-flash-up': {
          '0%': { backgroundColor: 'rgba(38, 166, 154, 0.2)' },
          '100%': { backgroundColor: 'transparent' }
        },
        'price-flash-down': {
          '0%': { backgroundColor: 'rgba(239, 83, 80, 0.2)' },
          '100%': { backgroundColor: 'transparent' }
        },
        'toast-slide-in': {
          '0%': { transform: 'translateX(100%)', opacity: '0' },
          '100%': { transform: 'translateX(0)', opacity: '1' }
        }
      },
      boxShadow: {
        'card': '0 1px 3px rgba(0, 0, 0, 0.3)',
        'card-hover': '0 4px 8px rgba(0, 0, 0, 0.4)',
        'dropdown': '0 4px 12px rgba(0, 0, 0, 0.5)'
      }
    },
  },
  plugins: [],
}
