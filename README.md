# 🪨 pkm

**A CLI for personal knowledge management and LLM wikis. No bulky app. Just the CLI.**

Your knowledge base is a folder of notes. pkm lets you and your AI agents
query, maintain and reorganize it directly from the terminal — on your laptop,
over SSH, or in CI. One binary, no desktop app, daemon, server or index required.

It understands `[[wikilinks]]`, `[markdown](links.md)`, YAML frontmatter,
tags and tasks, and reads `.org` files too. Use it with an existing vault or
any linked collection of notes.

```bash
pkm backlinks "Projects/recipe.md"   # what references this file?
pkm rename "recipe" "cookbook"   # rename it and update links
pkm tasks --overdue --format json    # what needs attention?
```

[Obsidian compatibility spec](spec.md) · [Embedded Dataview DQL](dql/README.md) · [CLI manual](docs/CLI.md) · [Compatibility](docs/COMPATIBILITY.md) ·
[Releases](https://github.com/combinatrix-ai/pkm/releases)

> A *pkm* is someone who strikes obsidian and flint into usable blades.
> This one works a directory of markdown into something an agent can wield.

## Why this exists

Plain text tools can find words. Maintaining a knowledge base also means
understanding how notes, topics, metadata and tasks fit together. pkm
brings those operations to the terminal, for people and agents alike:

- 🔎 **Structured queries** — filter and sort notes by frontmatter, inline fields, tags and computed link counts with `query`; discover available fields with `fields`.
- 🔗 **The link graph** — inspect incoming and outgoing links, find orphans and hubs, and detect broken links to notes, headings and block IDs.
- ✂️ **Link-safe refactors** — `rename` and `move` update links in both syntaxes; move whole directories and preview changes with `--dry-run`.
- 🩹 **Link repair planning** — `repair-links --dry-run` proposes repairs where the filesystem supplies unambiguous evidence.
- 🏷️ **Tags and topics** — list tags, follow nested topics with `backlinks '#tag'` and `context '#tag'`, and use `demote` to turn topic-only wikilinks into tags.
- ✅ **Tasks** — create, filter and update checkboxes across the tree, with dates, priorities, recurring markers and configurable statuses.
- 📅 **Daily notes** — create or retrieve a dated note and expand Core Templates or supported Templater date expressions.
- 📇 **Frontmatter** — read, set and delete typed YAML fields from the shell.
- 🩺 **Vault health** — find broken links, orphans, duplicate names and stubs; enforce frontmatter and required-heading rules by path or metadata, across the vault or only changed files.
- 🧠 **Context for agents** — get a note's content and relationships in one call, or focus on a search-hit line with its section and heading outline. JSON output and an embedded agent skill make automation straightforward.
- 🔐 **External references** — keep `pkm://` references in notes and resolve values through separately configured provider commands when needed.

Tags are Unicode-aware (`#café` and `#parent/child` both work), and one
malformed YAML header never aborts a whole-vault scan. The
[CLI manual](docs/CLI.md) covers all commands, options, configuration and
behavioral limits.

## Install

```bash
curl -fsSL https://raw.githubusercontent.com/combinatrix-ai/pkm/v0.1.0/install.sh | sh
```

Installs to `~/.local/bin`, verifies archive checksums, and registers the
embedded skill with supported agent hosts it finds. Release binaries support
macOS and Linux on arm64 and x86_64. See the [installation manual](docs/CLI.md#install)
for direct downloads, provenance verification, installer options and source builds.

`pkm self-update` verifies release provenance and installs updates.
Interactive commands check for updates in the background at most once every
six hours; set `PKM_NO_UPDATE_CHECK=1` to disable that check. Notes are
not sent. Configured external-reference providers may also use the network.

## Quick start

```bash
cd ~/your-notes
pkm init             # writes pkm.yaml — defaults are sane
pkm lint             # how healthy is this vault?
pkm tasks --overdue  # what slipped?
```

`pkm init` writes two files: the config, and the `Templates/daily.md` the
config points at, so `pkm daily` works straight afterwards. A template you
already have is never overwritten, `--force` included — that flag is about
replacing pkm's config, not your writing.

`pkm init` is one-time; after that, pkm walks up from wherever you are
to find the nearest `pkm.yaml`, so commands work from any
subdirectory.

```bash
pkm dql 'TABLE messages FROM "Diary" SORT file.day' --format json
pkm query --where 'status=open' --sort inlinks:desc
pkm context "Projects/recipe.md" --line 12 --format json
pkm move Projects/recipe Archive/recipe --dry-run
```

## Works with your notes

| Ecosystem | Support | What pkm understands |
|---|---|---|
| **Obsidian** | Feature subsets | Native links, properties, tags, daily notes, Tasks/Templater conventions and embedded Dataview DQL; see [spec.md](spec.md) for boundaries |
| **Foam** | ✅ | Wikilinks and markdown links, YAML frontmatter, and tags |
| **Dendron** | ✅ links | Dot-hierarchy filenames and links with or without `.md`; frontmatter IDs are read but do not resolve links |
| **Zettelkasten** | ✅ | ID-prefixed filenames and bare wikilinks |
| **Plain markdown / SSGs** | ✅ | Markdown links and YAML frontmatter in `.md`, `.markdown`, and `.mdx` files |
| **Logseq** | ⚠️ file-level | Page references, properties, tasks, and dates; block identity is intentionally ignored |
| **Roam exports** | ⚠️ file-level | Page references, macros, and tasks; block identity and attributes are not modelled |
| **org-mode / org-roam** | ⚠️ read support | Links, IDs, global heading links, properties, tags, todos, and planning dates; write commands still emit markdown |

Every claim above is pinned by a fixture test. See
**[the full compatibility matrix](docs/COMPATIBILITY.md)** for exact syntax,
plugin coverage, configuration, and known gaps.

## Use alongside ripgrep

Use [ripgrep](https://github.com/BurntSushi/ripgrep) for full-text search, then
pkm to inspect structure:

```bash
rg -n "some phrase" Notes
pkm context "Notes/Example.md" --line 42 --format json
```

pkm works directly on files. App services such as Sync, Publish,
workspaces and plugin execution remain outside its scope. See the
[compatibility matrix](docs/COMPATIBILITY.md) for supported syntax and limits.

## Status

Alpha. The documented CLI features are implemented and tested, but the CLI
surface may still shift before 1.0.

pkm began as a Python tool and was rewritten in Rust so that installing
it would not mean standing up an interpreter first. The two implementations
were developed side by side against
[one contract expressed as data](tests/contract/), which is how the port was
checked rather than hoped about; the Python one has since been retired, and
the cases outlived it.

## Development

```bash
git clone https://github.com/combinatrix-ai/pkm.git
cd pkm

cargo test          # unit tests, plus the contract suite against the binary
cargo clippy --all-targets -- -D warnings
cargo fmt --check
```

## License

MIT

---

<p align="center">
  <i>Built for agents, by an agent.</i> 🐾
</p>
