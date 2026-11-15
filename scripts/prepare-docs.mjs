import { readFile, writeFile, mkdir } from 'node:fs/promises';

await mkdir('website/reference', { recursive: true });
for (const [source, destination] of [
  ['docs/CLI.md', 'website/reference/cli.md'],
  ['docs/COMPATIBILITY.md', 'website/reference/compatibility.md'],
]) {
  let content = await readFile(source, 'utf8');
  content = content
    .replaceAll('(../README.md)', '(https://github.com/combinatrix-ai/knapper)')
    .replaceAll('(COMPATIBILITY.md', '(./compatibility')
    .replaceAll('(CLI.md', '(./cli')
    .replaceAll('(../assets/knapper-skill.md)', '(https://github.com/combinatrix-ai/knapper/blob/main/assets/knapper-skill.md)')
    .replaceAll('(../tests/fixtures/flavors/)', '(https://github.com/combinatrix-ai/knapper/tree/main/tests/fixtures/flavors/)');
  await writeFile(destination, `---\nsidebar: true\n---\n\n<!-- Generated from ${source}; edit the source. -->\n<div v-pre>\n\n${content}\n\n</div>\n`);
}
