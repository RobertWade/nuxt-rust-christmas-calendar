// filepath: /Users/robertwade/development/projects/nuxt-rust-christmas-calendar/web/tailwind.config.ts
import type { Config } from 'tailwindcss';

export default {
  content: [
    './app/**/*.{vue,js,ts}',
    './components/**/*.{vue,js,ts}',
    './layouts/**/*.{vue,js,ts}',
    './pages/**/*.{vue,js,ts}',
    './plugins/**/*.{js,ts}',
    './nuxt.config.{js,ts}',
  ],
  theme: {
    extend: {
      colors: {
        primary: '#C62828',
        secondary: '#F5C453',
        background: '#FFF8F0',
        text: '#2E1B0E',
        accent: '#A50000',
      },
    },
  },
  plugins: [],
} satisfies Config;