import { readFile, writeFile, mkdir, rm, readdir } from 'node:fs/promises';
import path from 'node:path';

const REPO = 'https://github.com/combinatrix-ai/pkm';
const DOCS = 'docs';
const SITE = 'website';

// Where each published docs file lives on the site. Anything else in the
// repository is linked on GitHub.
function sitePath(docsPath) {
  if (docsPath === 'README.md') return 'guide/index.md';
  if (docsPath === 'COMPATIBILITY.md') return 'reference/compatibility.md';
  if (/^(guide|reference)\/.+\.md$/.test(docsPath)) {
    return docsPath.replace(/README\.md$/, 'index.md');
  }
  return null;
}

async function walk(dir) {
  const files = [];
  for (const entry of await readdir(path.join(DOCS, dir), { withFileTypes: true })) {
    const relative = path.posix.join(dir, entry.name);
    if (entry.isDirectory()) files.push(...await walk(relative));
    else if (entry.name.endsWith('.md')) files.push(relative);
  }
  return files.sort();
}

// Rewrites relative link destinations outside fenced and inline code.
// `publish` maps a docs-relative path to its site-relative path, or null to
// link the file on GitHub.
function rewriteLinks(content, source, publish) {
  const link = /\]\(([^)\s]+)((?:\s+"[^"]*")?)\)/g;
  const retarget = (match, target, title) => {
    if (/^(#|[a-z][a-z0-9+.-]*:)/i.test(target)) return match;
    const [file, fragment] = target.split('#');
    const repoPath = path.posix.normalize(path.posix.join(DOCS, path.posix.dirname(source), file));
    const published = repoPath.startsWith(`${DOCS}/`) ? publish(repoPath.slice(DOCS.length + 1)) : null;
    const suffix = fragment ? `#${fragment}` : '';
    if (published) return `](${published}${suffix}${title})`;
    const kind = file.endsWith('/') ? 'tree' : 'blob';
    return `](${REPO}/${kind}/main/${repoPath}${suffix}${title})`;
  };
  let fence = null;
  return content.split('\n').map((line) => {
    const marker = line.match(/^\s*(`{3,}|~{3,})/)?.[1];
    if (marker && (!fence || marker.startsWith(fence))) {
      fence = fence ? null : marker;
      return line;
    }
    if (fence) return line;
    return line.split(/((`+)[^`]?(?:.*?[^`])?\2(?!`))/).map((part, i) => (i % 3 === 0 ? part.replace(link, retarget) : i % 3 === 1 ? part : '')).join('');
  }).join('\n');
}

await rm(path.join(SITE, 'guide'), { recursive: true, force: true });
await rm(path.join(SITE, 'reference'), { recursive: true, force: true });

const sources = ['README.md', 'COMPATIBILITY.md', ...await walk('guide'), ...await walk('reference')];
const full = [];
for (const source of sources) {
  const destination = sitePath(source);
  const content = await readFile(path.join(DOCS, source), 'utf8');
  const output = path.join(SITE, destination);
  await mkdir(path.dirname(output), { recursive: true });
  const onSite = (docsPath) => {
    const published = sitePath(docsPath);
    if (!published) return null;
    const relative = path.posix.relative(path.posix.dirname(destination), published);
    return relative.startsWith('.') ? relative : `./${relative}`;
  };
  // v-pre keeps template examples such as {{title}} out of Vue's hands.
  await writeFile(output, `<!-- Generated from ${DOCS}/${source}; edit the source. -->\n<div v-pre>\n\n${rewriteLinks(content, source, onSite)}\n\n</div>\n`);
  full.push(`Source: ${REPO}/blob/main/${DOCS}/${source}\n\n${rewriteLinks(content, source, () => null).trimEnd()}`);
}

// One file with every page, for agents that would rather fetch once. Links
// are absolute because the pages no longer sit beside each other.
await writeFile(path.join(SITE, 'public', 'llms-full.txt'), `${full.join('\n\n---\n\n')}\n`);
