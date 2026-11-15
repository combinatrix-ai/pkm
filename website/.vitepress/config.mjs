import { defineConfig } from 'vitepress';
export default defineConfig({
  title: 'knapper',
  description: 'Query, create, and reorganize connected Markdown notes from your terminal.',
  base: '/knapper/',
  cleanUrls: true,
  appearance: false,
  head: [['meta', { name: 'theme-color', content: '#216546' }]],
  themeConfig: {
    siteTitle: 'knapper /',
    nav: [
      { text: 'Quick start', link: '/guide/quick-start' },
      { text: 'CLI', link: '/reference/cli' },
      { text: 'GitHub', link: 'https://github.com/combinatrix-ai/knapper' },
    ],
    sidebar: [
      { text: 'Get started', items: [{ text: 'Quick start', link: '/guide/quick-start' }] },
      { text: 'Reference', items: [
        { text: 'CLI manual', link: '/reference/cli' },
        { text: 'Template variables', link: '/reference/cli#template-variables' },
        { text: 'Compatibility', link: '/reference/compatibility' },
      ] },
    ],
    search: { provider: 'local' },
    outline: [2, 3],
    footer: { message: 'Markdown files in. Markdown files out.', copyright: 'MIT licensed · Combinatrix' },
  },
});
