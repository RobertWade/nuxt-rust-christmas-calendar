import { defineNuxtConfig } from 'nuxt/config';

export default defineNuxtConfig({
  devtools: { enabled: true },
  css: ['./app/assets/css/tokens.css','./app/assets/css/main.css'],
  
  future: {
    compatibilityVersion: 4,
  },

  compatibilityDate: '2025-04-11',
  modules: [
    '@nuxt/eslint',
    '@nuxt/fonts',
    '@nuxt/icon',
    '@nuxt/image',
    '@nuxt/test-utils',
    '@nuxtjs/tailwindcss',
    '@vueuse/nuxt'
  ],
  app: {
    head: {
      meta: [
        {
          name: 'viewport',
          content: 'width=device-width, initial-scale=1, maximum-scale=1'
        }
      ]
    }
  },

  tailwindcss: {
    configPath: 'tailwind.config.ts'
  },

  // expose to network on dev
  devServer: {
    host: '0.0.0.0',
    port: 3000
  }
});