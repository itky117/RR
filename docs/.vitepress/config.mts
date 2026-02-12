import { defineConfig } from 'vitepress'

export default defineConfig({
  title: 'RR',
  description: 'An optimizing compiler from RR to R',
  base: '/RR/',
  themeConfig: {
    nav: [
      { text: 'Home', link: '/' },
      { text: 'Guide', link: '/getting-started' },
      { text: 'Reference', link: '/language' },
      { text: 'Internals', link: '/compiler-pipeline' },
    ],

    sidebar: [
      {
        text: 'Overview',
        items: [
          { text: 'Docs Home', link: '/' },
        ],
      },
      {
        text: 'Guide',
        items: [
          { text: 'Getting Started', link: '/getting-started' },
          { text: 'CLI Reference', link: '/cli' },
          { text: 'Configuration', link: '/configuration' },
        ],
      },
      {
        text: 'Reference',
        items: [
          { text: 'Language Reference (Code-Based)', link: '/language' },
          { text: 'Compatibility & Limits', link: '/compatibility' },
        ],
      },
      {
        text: 'Internals',
        items: [
          { text: 'Compiler Pipeline', link: '/compiler-pipeline' },
          { text: 'IR Model (HIR & MIR)', link: '/ir-model' },
          { text: 'Tachyon Optimizer', link: '/optimization' },
          { text: 'Runtime & Errors', link: '/runtime-and-errors' },
        ],
      },
      {
        text: 'Development',
        items: [
          { text: 'Testing & QA', link: '/testing' },
        ],
      },
    ],

    socialLinks: [
      { icon: 'github', link: 'https://github.com/Feralthedogg/RR' },
    ],

    search: {
      provider: 'local',
    },

    outline: 'deep',
  },
})
