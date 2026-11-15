<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { withBase } from 'vitepress';
const copied = ref(false);
const copyError = ref(false);
const install = 'curl -fsSL https://raw.githubusercontent.com/combinatrix-ai/knapper/v0.1.0/install.sh | sh';
const examples = [
  { name: 'Create', command: 'knapper new "Field notes" \\\n  --template Templates/Project.md \\\n  --folder Projects', result: 'Created: Projects/Field notes.md', caption: 'Start with your template, title, and today’s date.' },
  { name: 'Query', command: "knapper query --where 'status=active' --format paths", result: 'Projects/Field notes.md', caption: 'Find notes by metadata, tags, and connections.' },
  { name: 'Refactor', command: 'knapper rename "Field notes" "Research notes" --dry-run', result: '', caption: 'Preview the rename and link updates before writing.' },
  { name: 'Tasks', command: 'knapper tasks --overdue --format json', result: '', caption: 'Find overdue tasks and pass structured output to your agent.' },
];
const active = ref(0);
const hovering = ref(false);
const focused = ref(false);
const paused = ref(false);
const reducedMotion = ref(false);
let timer;
let motion;
function selectExample(index) { active.value = index; }
function navigateExample(event) {
  const moves = { ArrowRight: 1, ArrowLeft: -1 };
  if (!(event.key in moves) && !['Home', 'End'].includes(event.key)) return;
  event.preventDefault();
  active.value = event.key === 'Home' ? 0 : event.key === 'End' ? examples.length - 1 : (active.value + moves[event.key] + examples.length) % examples.length;
  event.currentTarget.querySelectorAll('[role="tab"]')[active.value].focus();
}
function motionChanged(event) { reducedMotion.value = event.matches; }
onMounted(() => {
  motion = window.matchMedia('(prefers-reduced-motion: reduce)');
  reducedMotion.value = motion.matches;
  motion.addEventListener('change', motionChanged);
  timer = setInterval(() => {
    if (!hovering.value && !focused.value && !paused.value && !reducedMotion.value && !document.hidden) {
      active.value = (active.value + 1) % examples.length;
    }
  }, 6000);
});
onUnmounted(() => { clearInterval(timer); motion?.removeEventListener('change', motionChanged); });
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
        <h1>App-less personal<br>knowledge management.</h1>
        <p class="hero-description">Connected Markdown notes, managed from your terminal.<br>For you and your AI agents.</p>
        <div class="hero-actions">
          <a class="primary-link" :href="withBase('/guide/quick-start')">Quick start <span aria-hidden="true">↗</span></a>
          <a class="text-link" :href="withBase('/reference/cli')">CLI manual <span aria-hidden="true">→</span></a>
        </div>
      </div>
      <div class="session" aria-label="Example commands" @mouseenter="hovering = true" @mouseleave="hovering = false" @focusin="focused = true" @focusout="focused = $event.currentTarget.contains($event.relatedTarget)">
        <div class="session-heading"><span>~/notes</span><button class="rotation-control" :aria-pressed="paused" @click="paused = !paused">{{ paused ? 'Resume' : 'Pause' }}</button></div>
        <div class="example-tabs" role="tablist" aria-label="CLI examples" @keydown="navigateExample">
          <button v-for="(example, index) in examples" :id="`example-tab-${index}`" :key="example.name" role="tab" :aria-selected="active === index" :aria-controls="`example-panel-${index}`" :tabindex="active === index ? 0 : -1" @click="selectExample(index)">{{ example.name }}</button>
        </div>
        <div v-for="(example, index) in examples" v-show="active === index" :id="`example-panel-${index}`" :key="example.name" role="tabpanel" :aria-labelledby="`example-tab-${index}`" tabindex="0">
          <pre><span class="prompt">$</span> {{ example.command }}<template v-if="example.result">
<span class="output">{{ example.result }}</span></template></pre>
          <div class="session-foot">{{ example.caption }}</div>
        </div>
      </div>
    </section>

    <section class="compatibility-strip" aria-label="Supported note ecosystems">
      <p>Your notes, from the tools you know.</p>
      <ul>
        <li>Obsidian</li><li>Foam</li><li>Dendron</li>
        <li>Logseq <small>file-level</small></li>
        <li>Roam <small>file-level</small></li>
        <li>Org-mode <small>read support</small></li>
      </ul>
      <a class="text-link" :href="withBase('/reference/compatibility')">See supported formats and limits →</a>
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
