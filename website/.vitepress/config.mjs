import { readdirSync } from 'node:fs';
import { defineConfig } from 'vitepress';

const commands = readdirSync(new URL('../../docs/reference/commands', import.meta.url))
  .filter((file) => file.endsWith('.md'))
  .map((file) => file.slice(0, -3))
  .sort();

export default defineConfig({
  title: 'pkm',
  description: "Your notes don't need an app anymore. pkm gives your agent what the app knew about your Markdown: links, metadata, tasks, and safe renames.",
  base: process.env.PKM_SITE_BASE || '/pkm/',
  cleanUrls: true,
  appearance: false,
  head: [['meta', { name: 'theme-color', content: '#216546' }]],
  themeConfig: {
    siteTitle: 'pkm /',
    nav: [
      { text: 'Quick start', link: '/guide/quick-start' },
      { text: 'Guides', link: '/guide/' },
      { text: 'CLI', link: '/reference/' },
      { text: 'GitHub', link: 'https://github.com/combinatrix-ai/pkm' },
    ],
    sidebar: [
      { text: 'Get started', items: [
        { text: 'Quick start', link: '/guide/quick-start' },
        { text: 'Install', link: '/guide/install' },
        { text: 'Working with agents', link: '/guide/agents' },
      ] },
      { text: 'Guides', items: [
        { text: 'Querying notes', link: '/guide/querying' },
        { text: 'Hard links and soft tags', link: '/guide/links-and-tags' },
        { text: 'Broken links', link: '/guide/link-health' },
        { text: "Renames that don't break anything", link: '/guide/refactoring' },
        { text: 'Repairing links', link: '/guide/repairing-links' },
        { text: 'Tasks', link: '/guide/tasks' },
        { text: 'Templates and daily notes', link: '/guide/templates' },
        { text: 'Lint rules', link: '/guide/lint' },
        { text: 'External references', link: '/guide/external-references' },
        { text: 'Configuration', link: '/guide/configuration' },
      ] },
      { text: 'Reference', items: [
        { text: 'CLI reference', link: '/reference/' },
        { text: 'Commands', collapsed: true, items: commands.map((name) => ({ text: name, link: `/reference/commands/${name}` })) },
        { text: 'Compatibility', link: '/reference/compatibility' },
      ] },
    ],
    search: { provider: 'local' },
    outline: [2, 3],
    footer: { message: 'Markdown files in. Markdown files out.', copyright: 'MIT licensed · Combinatrix' },
  },
});
