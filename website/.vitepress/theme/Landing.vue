<script setup>
import { ref } from 'vue';
import { withBase } from 'vitepress';
const copied = ref(false);
const copyError = ref(false);
const install = 'curl -fsSL https://raw.githubusercontent.com/combinatrix-ai/knapper/v0.1.0/install.sh | sh';
async function copyInstall() {
  try {
    await navigator.clipboard.writeText(install);
    copied.value = true;
    copyError.value = false;
    setTimeout(() => { copied.value = false; }, 2000);
  } catch { copyError.value = true; }
}
</script>

<template>
  <main class="landing">
    <section class="hero">
      <div>
        <p class="eyebrow">A CLI for connected notes</p>
        <h1>Your notes.<br>Your terminal.</h1>
        <p class="hero-description">Query, create, and reorganize Markdown notes.<br>Built for people and their agents.</p>
        <div class="hero-actions">
          <a class="primary-link" :href="withBase('/guide/quick-start')">Quick start <span aria-hidden="true">↗</span></a>
          <a class="text-link" :href="withBase('/reference/cli')">CLI manual <span aria-hidden="true">→</span></a>
        </div>
      </div>
      <div class="session" aria-label="Example commands for a synthetic notes folder">
        <div class="session-heading"><span>~/notes</span><span>EXAMPLE COMMANDS</span></div>
        <pre><span class="prompt">$</span> knapper new "Field notes" \
    --template Templates/Project.md \
    --folder Projects
<span class="output">Created: Projects/Field notes.md</span>

<span class="comment"># Follow the incoming links</span>
<span class="prompt">$</span> knapper backlinks "Projects/Field notes.md"

<span class="comment"># Give your agent structured output</span>
<span class="prompt">$</span> knapper tasks --overdue --format json</pre>
        <div class="session-foot">Plain files. Explicit commands. No index to maintain.</div>
      </div>
    </section>

    <section class="workflow" aria-labelledby="workflow-title">
      <div class="section-intro"><h2 id="workflow-title">Work with the connections.</h2><p>Text search finds words. knapper understands the links, metadata, and tasks around them.</p></div>
      <div class="workflow-rows">
        <article><span class="number">01</span><div><h3>Ask your notes</h3><p>Filter by status, tags, or link counts. Get a note's content and context in a single command.</p></div><code>knapper query --where 'status=active'</code></article>
        <article><span class="number">02</span><div><h3>Give new notes a shape</h3><p>Start with your template. Fill the title and date, then check the result with your vault's lint rules.</p></div><code>knapper new "Field notes" --template Project</code></article>
        <article><span class="number">03</span><div><h3>Move without losing references</h3><p>Preview a rename or move. knapper updates the links that point to the changed files.</p></div><code>knapper rename "Old" "New" --dry-run</code></article>
      </div>
    </section>

    <section class="install-section" aria-labelledby="install-title">
      <div><h2 id="install-title">One binary.<br>A folder of notes.</h2><p>macOS and Linux. Markdown and Org.<br>Works from your laptop, over SSH, or in CI.</p><a class="text-link" :href="withBase('/reference/compatibility')">Check compatibility →</a></div>
      <div class="install-box"><div class="install-heading"><span>Install knapper</span><button @click="copyInstall">{{ copied ? 'Copied' : 'Copy' }}</button></div><pre tabindex="0"><code>{{ install }}</code></pre><p role="status">{{ copyError ? 'Copy unavailable. Select the command above.' : 'Already installed? Run knapper self-update.' }}</p><a href="https://github.com/combinatrix-ai/knapper/releases">Release notes and downloads ↗</a></div>
    </section>

    <section class="docs-section" aria-labelledby="docs-title"><h2 id="docs-title">Keep the files. Bring your tools.</h2><p>Use an existing Obsidian vault or any linked folder of Markdown. Run <code>knapper skill</code> to give your agent the command reference.</p><div class="doc-links"><a :href="withBase('/reference/cli')">CLI reference <span>Commands, configuration, and examples →</span></a><a :href="withBase('/reference/cli#template-variables')">Template variables <span>Date, time, title, and Templater support →</span></a><a href="https://github.com/combinatrix-ai/knapper">Source code <span>Read the code, report an issue, contribute ↗</span></a></div></section>
  </main>
</template>
