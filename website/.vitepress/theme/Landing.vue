<script setup>
import { ref } from 'vue';
import { withBase } from 'vitepress';
import examples from '../../demo/examples.json';

const { conversations, snippets, rules_config: rulesConfig } = examples;
const active = ref(0);

function navigateExample(event) {
  const moves = { ArrowRight: 1, ArrowLeft: -1 };
  if (!(event.key in moves)) return;
  event.preventDefault();
  active.value = (active.value + moves[event.key] + conversations.length) % conversations.length;
  event.currentTarget.querySelectorAll('[role="tab"]')[active.value].focus();
}

const agentPrompt = 'Install knapper from https://github.com/combinatrix-ai/knapper, run `knapper skill`, and use it whenever you work with my notes in ~/notes.';
const install = 'curl -fsSL https://raw.githubusercontent.com/combinatrix-ai/knapper/v0.1.0/install.sh | sh';
const copied = ref('');
async function copy(name, text) {
  try {
    await navigator.clipboard.writeText(text);
    copied.value = name;
    setTimeout(() => { copied.value = ''; }, 2000);
  } catch { copied.value = `${name}-failed`; }
}

const replaced = [
  { app: 'Backlinks pane', commands: 'backlinks · links · context', say: 'What have I written about this?', guide: '/guide/agents' },
  { app: 'Rename with link updates', commands: 'rename · move', say: 'Move the 2025 projects into Archive.', guide: '/guide/refactoring' },
  { app: 'Query plugins', commands: 'query · fields · frontmatter · tags', say: 'Which projects are still active?', guide: '/guide/querying' },
  { app: 'Task plugins', commands: 'tasks · tasks new · tasks done', say: "What's overdue? Mark the first one done.", guide: '/guide/tasks' },
  { app: 'Daily notes and templates', commands: 'daily · new', say: "Log this in today's note.", guide: '/guide/templates' },
  { app: 'Graph view', commands: 'hubs · orphans', say: 'Which notes connect to nothing?', guide: '/guide/querying' },
  { app: 'Broken link checks', commands: 'broken-links · lint', say: 'Is anything broken?', guide: '/guide/link-health' },
];

const more = [
  { title: 'Repairs backed by evidence',
    text: 'Links also break while nobody is looking: a folder moved in Finder, an export with stale paths. knapper proposes a fix only where the filesystem settles it, and reports the rest instead of guessing.',
    snippet: snippets.repair, guide: '/guide/repairing-links', link: 'How repairs are decided' },
  { title: "Links that promise, tags that don't",
    text: 'After a few years a vault is full of [[brackets]] used as a highlighter, like [[to-read]], with no note behind them. [[X]] is a promise that a note exists; #X is a topic that needs none. demote moves one to the other and reports what a tag cannot hold.',
    snippet: snippets.demote, guide: '/guide/links-and-tags', link: 'Hard links and soft tags' },
  { title: 'House rules for your vault',
    text: 'Say what a project note must contain, by path or by metadata. Agents keep to your structure because the linter tells them when they did not.',
    config: rulesConfig, snippet: snippets.rules, guide: '/guide/lint', link: 'Lint rules' },
  { title: 'Names in notes, values elsewhere',
    text: 'An address or a licence key belongs in your notes by name, not by value. Write it as a knapper:// link and the value stays in your password manager until someone asks for that one item.',
    snippet: snippets.refs, guide: '/guide/external-references', link: 'External references',
    works: [['1password', '1Password'], ['bitwarden', 'Bitwarden']] },
  { title: 'Whole folders move too',
    text: 'Archiving a year of projects is one command. Every note inside moves, and every link that pointed into the folder follows it.',
    snippet: snippets.movedir, guide: '/guide/refactoring', link: 'Renames and moves' },
  { title: 'One call, the whole picture',
    text: 'Before your agent touches a note it can ask for everything around it: the text, what it links to, what links back, and its headings. One call instead of five greps.',
    snippet: snippets.context, guide: '/guide/agents', link: 'Working with agents' },
];
</script>

<template>
  <main class="landing">
    <section class="hero">
      <div>
        <p class="eyebrow">Markdown + your agent</p>
        <h1>Your notes don't need<br>an app anymore.</h1>
        <p class="hero-description">Your agent already reads and writes Markdown. knapper gives it what the app knew: links, metadata, tasks, and renames that don't break anything.</p>
        <div class="hero-actions">
          <a class="primary-link" href="#install">Give it to your agent <span aria-hidden="true">↓</span></a>
          <a class="text-link" :href="withBase('/guide/quick-start')">Quick start <span aria-hidden="true">→</span></a>
        </div>
      </div>
      <div class="session" aria-label="Example conversations">
        <div class="session-heading"><span>your agent · ~/notes</span><span>no app running</span></div>
        <div class="example-tabs" role="tablist" aria-label="Examples" @keydown="navigateExample">
          <button v-for="(example, index) in conversations" :id="`example-tab-${index}`" :key="example.name" role="tab" :aria-selected="active === index" :aria-controls="`example-panel-${index}`" :tabindex="active === index ? 0 : -1" @click="active = index">{{ example.name }}</button>
        </div>
        <div v-for="(example, index) in conversations" v-show="active === index" :id="`example-panel-${index}`" :key="example.name" role="tabpanel" :aria-labelledby="`example-tab-${index}`" tabindex="0">
          <div class="turns">
            <div class="turn you"><span class="who">You</span><p>{{ example.you }}</p></div>
            <div class="turn tool"><span class="who">Agent runs</span><pre><span class="prompt">$</span> {{ example.command }}
<span class="output">{{ example.result }}</span></pre></div>
            <div class="turn agent"><span class="who">Agent</span><p>{{ example.agent }}</p></div>
          </div>
          <div class="session-foot">{{ example.caption }}</div>
        </div>
      </div>
    </section>

    <section class="band recipe" aria-labelledby="why-title">
      <div>
        <p class="eyebrow">Why now</p>
        <h2 id="why-title">The app was the interface.<br>Now the agent is.</h2>
        <p>Note apps exist so a person can click through a folder of text files. If you ask an agent instead, the window has nothing left to do. What the agent still lacks is the app's understanding of your notes. That part is knapper.</p>
      </div>
      <div class="flows">
        <div class="flow old"><small>Before</small><div class="chain"><span>You</span><i>→</i><span class="gone">Notes app</span><i>→</i><span>Markdown</span></div></div>
        <div class="flow now"><small>Now</small><div class="chain"><span>You</span><i>→</i><span>Agent</span><i>→</i><span class="k">knapper</span><i>→</i><span>Markdown</span></div></div>
      </div>
    </section>

    <section class="band" aria-labelledby="grep-title">
      <p class="eyebrow">Why not just grep</p>
      <h2 id="grep-title">grep finds words.<br>Your notes are a graph.</h2>
      <p>Without knapper, your agent improvises a <code>sed</code> or <code>awk</code> one-liner for every structural change. It spends tokens composing a regex, runs it across your vault, and has no way to tell what it missed. knapper turns each of those into one command that knows what a link is.</p>
      <div class="versus">
        <div class="vs bad">
          <header>Agent with shell tools</header>
          <pre><b>$ mv Projects/Redesign.md Projects/Relaunch.md</b>
$ grep -rl "Redesign" . | xargs sed -i -E \
  's/\[\[(Projects\/)?Redesign(#[^]|]*)?(\|[^]]*)?\]\]/[[\1Relaunch\2\3]]/g'</pre>
          <p>A regex written from scratch, every time. This one skips <code>[the redesign](../Projects/Redesign.md)</code>, so that link is now broken and nothing says so.</p>
        </div>
        <div class="vs good">
          <header>Agent with knapper</header>
          <pre><b>$ {{ snippets.rename.command }}</b>
{{ snippets.rename.result }}</pre>
          <p>One call. Wikilinks and Markdown links are both rewritten. Add <code>--dry-run</code> to see the plan first.</p>
        </div>
      </div>
    </section>

    <section class="band" aria-labelledby="replaces-title">
      <p class="eyebrow">What it replaces</p>
      <h2 id="replaces-title">Everything the app did.<br>Minus the app.</h2>
      <p>Each pane and plugin you relied on is a command your agent can call. Queries answer in JSON.</p>
      <table class="replaces" role="table">
        <thead role="rowgroup"><tr role="row"><th role="columnheader">In the app</th><th role="columnheader">With knapper</th><th role="columnheader">You say</th><th role="columnheader"><span class="visually-hidden">Guide</span></th></tr></thead>
        <tbody role="rowgroup">
          <tr v-for="row in replaced" :key="row.app" role="row">
            <td role="cell">{{ row.app }}</td>
            <td role="cell"><code>{{ row.commands }}</code></td>
            <td role="cell">"{{ row.say }}"</td>
            <td role="cell"><a :href="withBase(row.guide)" :aria-label="`Guide: ${row.app}`">Guide →</a></td>
          </tr>
        </tbody>
      </table>
    </section>

    <section class="band" aria-labelledby="more-title">
      <p class="eyebrow">And more</p>
      <h2 id="more-title">Then the things it never did.</h2>
      <p>Years of notes go wrong in specific ways. These have no pane in any app.</p>
      <div class="more-list">
        <article v-for="item in more" :key="item.title">
          <h3>{{ item.title }}</h3>
          <p>{{ item.text }}</p>
          <pre v-if="item.config" class="config">{{ item.config }}</pre>
          <pre tabindex="0"><b>$ {{ item.snippet.command }}</b>
{{ item.snippet.result }}</pre>
          <ul v-if="item.works" class="works" aria-label="Works with">
            <li v-for="[icon, name] in item.works" :key="icon"><img :src="withBase(`/icons/${icon}.svg`)" alt="" width="18" height="18">{{ name }}</li>
            <li>or any CLI that prints a value</li>
          </ul>
          <a class="text-link" :href="withBase(item.guide)">{{ item.link }} →</a>
        </article>
      </div>
    </section>

    <section class="band keep" aria-labelledby="keep-title">
      <p class="eyebrow">Migration</p>
      <h2 id="keep-title">Keep the vault. Drop the app.</h2>
      <p>Point knapper at the folder you already have. Nothing is imported, converted, or indexed, so the app keeps working for as long as you still want it.</p>
      <ul>
        <li><img :src="withBase('/icons/obsidian.svg')" alt="" width="26" height="26">Obsidian</li>
        <li><img :src="withBase('/icons/logseq.svg')" alt="" width="26" height="26">Logseq</li>
        <li><img :src="withBase('/icons/foam.svg')" alt="" width="26" height="26">Foam</li>
        <li><img :src="withBase('/icons/dendron.svg')" alt="" width="26" height="26">Dendron</li>
        <li><img :src="withBase('/icons/roamresearch.svg')" alt="" width="26" height="26">Roam</li>
        <li><img :src="withBase('/icons/org.svg')" alt="" width="26" height="26">Org-mode</li>
      </ul>
      <a class="text-link" :href="withBase('/reference/compatibility')">What works with each format →</a>
    </section>

    <section id="install" class="band" aria-labelledby="install-title">
      <p class="eyebrow">Install</p>
      <h2 id="install-title">Hand it to your agent.</h2>
      <p>knapper ships its own command reference. Your agent installs it, reads <code>knapper skill</code>, and knows the rest.</p>
      <div class="install-grid">
        <div class="install-box lead">
          <div class="install-heading"><span>Paste this to your agent</span><button @click="copy('prompt', agentPrompt)">{{ copied === 'prompt' ? 'Copied' : 'Copy' }}</button></div>
          <pre tabindex="0">{{ agentPrompt }}</pre>
          <p role="status">{{ copied === 'prompt-failed' ? 'Copy unavailable. Select the text above.' : 'Works with Claude Code, Codex, or any agent that can run a shell command.' }}</p>
        </div>
        <div class="install-box">
          <div class="install-heading"><span>Or install it yourself</span><button @click="copy('install', install)">{{ copied === 'install' ? 'Copied' : 'Copy' }}</button></div>
          <pre tabindex="0"><code>{{ install }}</code></pre>
          <p role="status">{{ copied === 'install-failed' ? 'Copy unavailable. Select the command above.' : 'macOS and Linux. Already installed? Run knapper self-update.' }}</p>
        </div>
      </div>
      <div class="facts">
        <div>One binary<span>No daemon, server, or index to keep in sync.</span></div>
        <div>Runs anywhere<span>Laptop, SSH, CI. Wherever the folder is.</span></div>
        <div>Plain files<span>Markdown in, Markdown out. Org files are read too.</span></div>
        <div>Open source<span><a href="https://github.com/combinatrix-ai/knapper">Read the code</a>, report an issue, contribute.</span></div>
      </div>
    </section>

    <section class="band docs-section" aria-labelledby="docs-title">
      <h2 id="docs-title">Read the details.</h2>
      <div class="doc-links">
        <a :href="withBase('/guide/')">Guides <span>How each feature works, and why →</span></a>
        <a :href="withBase('/reference/')">CLI reference <span>Every command, argument, and option →</span></a>
        <a :href="withBase('/llms-full.txt')">llms-full.txt <span>All of the documentation in one file, for your agent →</span></a>
      </div>
      <p class="aside">Still like your editor? Keep it. knapper only works on the files.</p>
    </section>
  </main>
</template>
