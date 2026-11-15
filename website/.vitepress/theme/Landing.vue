<script setup>
import { ref, onMounted, onUnmounted } from 'vue';
import { withBase } from 'vitepress';
const copied = ref(false);
const copyError = ref(false);
const install = 'curl -fsSL https://raw.githubusercontent.com/combinatrix-ai/knapper/v0.1.0/install.sh | sh';
const examples = [
  { name: 'Create', command: 'knapper new "Field notes" \\\n  --template Templates/Project.md \\\n  --folder Projects', result: 'Created: Projects/Field notes.md', caption: 'Create a structured note from your template, then check it with lint.' },
  { name: 'Query', command: "knapper query --where 'status=active' --format paths", result: 'Projects/Field notes.md', caption: 'Find notes by metadata, tags, and link counts.' },
  { name: 'Refactor', command: 'knapper rename "Field notes" "Research notes" --dry-run', result: '', caption: 'Rename notes and update their links. Preview every change before writing.' },
  { name: 'Tasks', command: 'knapper tasks --overdue --format json', result: '', caption: 'Find overdue tasks and pass structured output to your agent.' },
];
const active = ref(0);
const hovering = ref(false);
const focused = ref(false);
const paused = ref(false);
const reducedMotion = ref(false);
const progress = ref(0);
const rotationDuration = 6000;
let frame;
let lastFrame;
let motion;
function selectExample(index) { active.value = index; progress.value = 0; }
function navigateExample(event) {
  const moves = { ArrowRight: 1, ArrowLeft: -1 };
  if (!(event.key in moves) && !['Home', 'End'].includes(event.key)) return;
  event.preventDefault();
  selectExample(event.key === 'Home' ? 0 : event.key === 'End' ? examples.length - 1 : (active.value + moves[event.key] + examples.length) % examples.length);
  event.currentTarget.querySelectorAll('[role="tab"]')[active.value].focus();
}
function motionChanged(event) { reducedMotion.value = event.matches; }
onMounted(() => {
  motion = window.matchMedia('(prefers-reduced-motion: reduce)');
  reducedMotion.value = motion.matches;
  motion.addEventListener('change', motionChanged);
  function tick(now) {
    const elapsed = lastFrame === undefined ? 0 : Math.min(now - lastFrame, 100);
    lastFrame = now;
    if (!hovering.value && !focused.value && !paused.value && !reducedMotion.value && !document.hidden) {
      progress.value += elapsed / rotationDuration;
      if (progress.value >= 1) selectExample((active.value + 1) % examples.length);
    }
    frame = requestAnimationFrame(tick);
  }
  frame = requestAnimationFrame(tick);
});
onUnmounted(() => { cancelAnimationFrame(frame); motion?.removeEventListener('change', motionChanged); });

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
          <div class="rotation-progress" aria-hidden="true"><span :style="{ transform: `scaleX(${progress})` }"></span></div>
        </div>
      </div>
    </section>

    <section class="compatibility-strip" aria-label="Supported note ecosystems">
      <p>Works with your notes.</p>
      <ul>
        <li><img :src="withBase('/icons/obsidian.svg')" alt="" width="26" height="26">Obsidian</li>
        <li><img :src="withBase('/icons/foam.svg')" alt="" width="26" height="26">Foam</li>
        <li><img :src="withBase('/icons/dendron.svg')" alt="" width="26" height="26">Dendron</li>
        <li><img :src="withBase('/icons/logseq.svg')" alt="" width="26" height="26">Logseq</li>
        <li><img :src="withBase('/icons/roamresearch.svg')" alt="" width="26" height="26">Roam</li>
        <li><img :src="withBase('/icons/org.svg')" alt="" width="26" height="26">Org-mode</li>
      </ul>
      <a class="text-link" :href="withBase('/reference/compatibility')">Compatibility and supported formats →</a>
    </section>

    <section class="install-section" aria-labelledby="install-title">
      <div><h2 id="install-title">One binary.<br>A folder of notes.</h2><p>macOS and Linux. Markdown and Org.<br>Works from your laptop, over SSH, or in CI.</p><a class="text-link" :href="withBase('/reference/compatibility')">Check compatibility →</a></div>
      <div class="install-box"><div class="install-heading"><span>Install knapper</span><button @click="copyInstall">{{ copied ? 'Copied' : 'Copy' }}</button></div><pre tabindex="0"><code>{{ install }}</code></pre><p role="status">{{ copyError ? 'Copy unavailable. Select the command above.' : 'Already installed? Run knapper self-update.' }}</p><a href="https://github.com/combinatrix-ai/knapper/releases">Release notes and downloads ↗</a></div>
    </section>

    <section class="docs-section" aria-labelledby="docs-title"><h2 id="docs-title">Keep the files. Bring your tools.</h2><p>Use an existing Obsidian vault or any linked folder of Markdown. Run <code>knapper skill</code> to give your agent the command reference.</p><div class="doc-links"><a :href="withBase('/reference/cli')">CLI reference <span>Commands, configuration, and examples →</span></a><a :href="withBase('/reference/cli#template-variables')">Template variables <span>Date, time, title, and Templater support →</span></a><a href="https://github.com/combinatrix-ai/knapper">Source code <span>Read the code, report an issue, contribute ↗</span></a></div></section>
  </main>
</template>
