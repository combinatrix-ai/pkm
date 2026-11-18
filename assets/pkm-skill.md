---
name: pkm
description: Query and edit note links, tasks, and metadata with pkm; use for structural queries, link-preserving moves, daily notes, or explicit pkm references. Use text search for prose.
---

# pkm

One binary that reads and writes a directory of markdown notes directly -- an
Obsidian vault, a Foam or Dendron workspace, a Zettelkasten, an org-roam
directory, or any folder of `.md` files that has grown links. Nothing needs to
be running. Interactive commands may check GitHub for updates every six hours;
`PKM_NO_UPDATE_CHECK=1` disables these checks. A provider command configured
for `resolve` may open its own connection.

Use it for the operations that need to understand a vault's *structure*, which
plain shell tools cannot do: the link graph, link-preserving renames, tasks,
frontmatter.

**Do not use it for full-text search.** pkm ships none and never will.
Prose -- a name, a phrase, a keyword -- belongs to `rg`, or to whatever search
your harness already gives you.

**Do reach for it instead of a regex over markdown syntax.** A vault's
structure looks regular enough to grep and is not: the pattern fires inside a
code fence, misses the aliased form, and cannot see what some other file says
about this one. If you are about to write one of the greps on the left, the
command on the right is the answer you actually wanted.

| Tempting | Ask pkm instead | What the regex gets wrong |
|---|---|---|
| `rg -o '\[\[' -g '*.md'` | `pkm links FILE`, `pkm backlinks FILE` | `![[embeds]]`, `[[X\|alias]]`, `[[X#heading]]` and `[text](x.md)` are links too; the ones inside code fences and `%%comments%%` are not. Backlinks are not greppable at all without resolving basenames and aliases first |
| `rg -l '^status:' -g '*.md'` | `pkm query --where status=open` | a body line may start the same way; the value may be quoted, a list, or a Dataview `status:: open`; and `query` filters and sorts on `inlinks`/`outlinks`/`broken` in the same breath |
| `rg -l '#project' -g '*.md'` | `pkm backlinks '#project'` | `#project/sub` nests, a `#` in a URL or a heading is not a tag, and a tag in code is not a tag |
| `rg -l 'Old Name' \| xargs sed -i ''` | `pkm rename 'Old Name' 'New Name'`, `pkm move FILE DIR/` | `sed` rewrites text but never the file itself, misses whichever link syntax you did not think of, mangles `[[X\|alias]]` and `[[X#heading]]`, and does not know that a markdown link to a name with a space in it is written `Old%20Name.md` or `<Old Name.md>` -- both of which keep their shape here. Both commands take `--dry-run` |
| `rg '\]\(.*\.md\)' -g '*.md'` | `pkm broken-links` | whether a target resolves depends on basenames, aliases, relative paths and `ignore_links` -- a regex can find link syntax but never tell you which links are broken |
| `rg '^\s*- \[ \]' -g '*.md'` | `pkm tasks --overdue`, `--due-to`, `--tag`, `--status` | the regex has no notion of a status character (`- [/]`, `- [-]`, custom ones), a due date, a tag, or an excluded subtree |

For a structural question, call the relevant pkm command directly. When
a prose search is needed to locate a note first, combine the tools:

```bash
rg -l "some phrase" Diary | head -3 | xargs -n1 pkm context --format json
```

If Obsidian itself is running on this machine, prefer the official `obsidian`
CLI (https://help.obsidian.md/cli) for anything it covers -- Bases, Sync,
Publish, workspaces. pkm is for everywhere that is not true.

## Setup

pkm walks up from the working directory to find the nearest
`pkm.yaml`, so commands work from any subdirectory once one exists.
The former `pkm.config.md` is not discovered or used.

```bash
cd ~/your-vault
pkm init          # writes pkm.yaml and Templates/daily.md
pkm config check  # validate pkm.yaml without scanning the vault
```

## Output formats

Query commands take `-f/--format`:

- `text` (default) — for a human
- `json` — parse this, not the text
- `paths` — bare paths, for piping

Mutating commands (`rename`, `move`, `frontmatter set`, `tasks new`) report
what they did; `rename` and `move` also accept `--format json` and
`--dry-run`.

**Always pass `--format json` when you intend to parse the result.**

## Finding notes

`query` is the general form; `orphans` and `hubs` are presets over it.
`broken-links` adds occurrence details and applies lint policies by default.

```bash
pkm query --where status=open --from Questions --sort inlinks:desc
pkm query --where inlinks=0                  # what orphans does
pkm query --where broken>0 --field broken    # without lint policies
pkm query --sort inlinks:desc --limit 10     # what hubs does
pkm fields                                   # what this vault can be filtered on
```

Frontmatter keys, Dataview inline fields (`[cost:: 42]`) and computed fields
share one namespace, so `status=open`, `cost>40` and `inlinks=0` all work the
same way. Operators: `=` `!=` `>` `<` `>=` `<=` `~` (contains), a bare name for
"has this field", `!name` for "does not".

Computed fields: `path` `title` `ext` `inlinks` `outlinks` `broken` `tags`
`words` `chars` `lines` `modified` `age`.

## Reading one note

`context` aggregates what would otherwise take five calls -- content, links,
backlinks, tags, inline fields, headings, tasks, stats:

```bash
pkm context "Notes/Lit Review.md" --format json
```

Trim it when the whole note is not needed:

```bash
pkm context FILE --no-content --no-backlinks   # structure only
pkm context FILE --max-content 2000            # truncated, marked as such
pkm context FILE --line 120                   # focused context, default -B 3 -A 3
pkm context FILE --line 120 --section         # smallest enclosing Markdown section
pkm context FILE --line 120 --outline-depth 2 # include nested outline entries
```

`--no-backlinks` skips building the link graph, which is the expensive part on
a large vault.

Focused `context --line N` addresses a physical 1-based line and returns a
line-numbered excerpt, the complete heading breadcrumb, and a first-layer
document map. `-B/--before` and `-A/--after` override the default three-line
window. `--section` replaces the window with the smallest enclosing Markdown
section and cannot be combined with explicit `-A` or `-B`; `--outline-depth 0`
hides the map. H1 children form the first layer when an H1 title exists,
otherwise the shallowest heading level does. Maps are compacted above twelve
entries and report their omission counts. `--no-content` keeps focus metadata
but omits the excerpt text, while `--max-content` truncates around the focused
line. Focused mode rejects org-mode notes until an org outline contract exists.

## Links

```bash
pkm links FILE                  # outgoing, both syntaxes
pkm backlinks FILE              # what references this file
pkm backlinks FILE -A 2 -B 2    # with surrounding lines, like grep
pkm orphans                     # nothing links here
pkm hubs --limit 10             # most linked-to
pkm broken-links                # links pointing nowhere
pkm broken-links --format json  # one record per occurrence, with position
```

`orphans` hides dotfolders; `--include-special` shows them. No other folder is
special by name: a template folder stays out of `orphans` and `lint` only by
being listed in `exclude:`, which is what `pkm init` writes.

`broken-links --format json` is occurrence-oriented, not a per-file summary:
each record carries `source`, `line`, `column` (1-based, in characters, and
`null` for org), `syntax`, `raw` (the link as written), `raw_target`, `target`
(what the resolver was asked), `reason`, `status` and `candidates`. It reads
the same scan as `repair-links`, then applies lint policies by default.
Use `--all` to see the unfiltered diagnostic scope.

`broken-links` respects the same enabled flags, source scopes, ordered path /
metadata rules and target patterns as ordinary lint. Use `broken-links --all`
for a diagnostic scan without those lint policies; top-level `exclude` and
`ignore_links` still apply. `repair-links` remains an unfiltered diagnostic.

## Hard links and soft tags

Two kinds of reference, and they mean different things:

- `[[X]]` is a **hard note reference**. It names a note, it is an edge in the
  graph, and a missing target is a broken link.
- `#X` is a **soft topic reference**. It labels a note with a subject, needs
  no note to exist, is never broken, and is never an orphan or a hub.

A `#tag` argument makes the soft one navigable, as a *virtual* subject:

```bash
pkm backlinks '#book-club'                 # notes and lines carrying it
pkm backlinks '#book-club' -A 2 --format json
pkm context '#book-club' --format json     # the whole topic, as one object
```

Quote it: an unquoted `#` starts a comment in most shells.

- A leading `#` is the **only** thing that selects a tag. Everything else is a
  note, so nothing changes for ordinary arguments.
- A tag covers the tags **nested** under it: `#work` finds `#work/events`.
  Every result names the tag it actually matched.
- Matching is otherwise **exact and case-sensitive**, with no Unicode
  normalisation — the same rule `tags --find` follows.
- Occurrences come from prose (`where: "inline"`) and from metadata
  (`where: "declared"`: YAML `tags:`, org `#+filetags:` and heading tags).
  Code fences, inline spans and `%%comments%%` hold no occurrences.
- Results are sorted by path then line, so the JSON is stable between runs.
- `context '#tag'` returns `kind: "tag"` with `notes`, `occurrences`,
  `nested_tags`, `tasks` and `stats`, and no `path` — nothing pretends a note
  exists. `--no-content` drops the line text, `--max-content` truncates it and
  `--no-tasks` drops the tasks; `--no-backlinks` has nothing to skip.

### Demoting a wikilink to a tag

For the vault that has been writing `[[book-club]]` for years as a label rather
than as a claim that such a note exists:

```bash
pkm demote "book-club" --dry-run --format json
pkm demote "book-club"
```

Only the **exact** form `[[book-club]]` becomes `#book-club`. Everything a tag
cannot hold is reported under `skipped`, with a file, a line and a reason, and
left exactly as it was: `alias`, `anchor`, `embed`, `path-qualified`,
`adjacent text` (`見た[[X]]の` would produce a different tag), `frontmatter`
and `org-mode`. Code and comments are not references and are neither rewritten
nor reported.

- `--dry-run` reports every proposed edit and writes nothing.
- A target that names a **note that exists** is refused: demoting it would
  drop a real reference. `--allow-existing-note` does it anyway.
- A target that cannot be spelled as a tag (`Daily Tasks`) is refused before
  anything is read; `--tag work/daily` names the tag to write instead.
- A path-qualified target (`Archives/X`) is refused: `demote` names a bare
  target. Use `--tag a/b` to write a nested tag.

Promotion the other way (`#X` back to `[[X]]`) is deliberately not
implemented: it would have to invent a note, choose where it lives, and turn a
label into a promise. Create the note and use `rename`.

## Renaming without breaking links

The reason to reach for pkm at all. Every inbound link is rewritten, in
both syntaxes, preserving link text, folder prefixes, `#anchors`, and the way
the target was written -- `Old%20Name.md` stays encoded and `<Old Name.md>`
stays angle-wrapped. External URLs and image embeds are left alone.

```bash
pkm rename "Recipe" "Cookbook"
pkm move "note.md" "Archive/"
pkm rename OLD NEW --dry-run    # preview first
```

Prefer `--dry-run` before a rename you are not certain about.

### Moving a directory

`move` takes a directory too. The subtree moves as it stands — notes,
attachments, sidecars, dotfiles, nested folders — and every link that
*resolves* into it follows.

```bash
pkm move "Projects/Recipe" "Archive/" --dry-run --format json
pkm move "Projects/Recipe" "Archive/" --format json
```

- The source is an **exact** vault-relative directory path, trailing slash
  optional. There is no name matching: `move Recipe Archive/` will not find
  `Projects/Recipe`.
- The result is always `DEST/<the directory's own name>`. It does not rename,
  and it refuses rather than merging into an existing directory.
- "Inbound" means resolution, not text. A bare `[[README]]` that meant some
  other README is left alone; one that meant the moved README is qualified so
  it still resolves. Relative links out of the subtree are recomputed; links
  within it are left alone when the structure keeps them working. Attachment
  and image links follow their files. Prose, code, `%%comments%%`, external
  URLs and `pkm://` references are never touched.
- JSON reports `kind`, `old_path`, `new_path`, `entries`, `notes`,
  `files_updated`, `links_updated`, `unsupported_links` and `warnings`.
  `--dry-run` adds `moves` (every file's old and new path) and `edits` (every
  link change, with line, before and after) — enough to act on without
  reading the vault again — and writes nothing.
- Both ends must be inside the real vault tree. A source or destination that
  reaches its directory through a **symlink** is refused, however
  vault-relative the path looks: following it would move the directory out of
  the vault and leave every rewritten link pointing nowhere. A symlinked
  *note* is different and is followed: rewriting a link inside one updates
  the file it points at and leaves the symlink a symlink.
- Rewritten inline links are percent-encoded for the path they now have. A
  directory called `Guide (v2)` gives `[a](Archive/Guide%20%28v2%29/Note.md)`,
  since an unencoded `)` would end the link early; `%`, `"`, `<`, `>` and `#`
  are encoded for the same reason. Wikilinks are not URL-like and are left
  plain; a `<...>` destination keeps its angle brackets.
- pkm does not rewrite org-mode links. If an inbound `.org` link points
  into the directory, the move **stops and changes nothing**, listing them
  under `unsupported_links`; `--allow-broken-org-links` proceeds anyway and
  still reports them. Run `--dry-run` first to see them.

Use `--dry-run` before every directory move: it is the cheap way to see
exactly which links a move would touch.

## Repairing links that broke somewhere else

`rename` and `move` keep links intact through a refactor pkm performs.
`repair-links` is the other half: the ones that broke while pkm was not
looking — a folder reorganised elsewhere, an exporter's stale path, an
encoding that survived a round trip.

```bash
pkm repair-links --dry-run                  # the plan, as text
pkm repair-links --dry-run --format json    # the plan, machine-readable
```

**V1 plans and nothing else.** `--dry-run` is required; omitting it exits 2
before the vault is read. There is no `--apply`, so never tell a user this
command fixed anything — hand them the plan.

Every occurrence is `safe`, `ambiguous` or `unresolved`. Only two bases are
ever safe, and both are structural:

- `unique-path-suffix` — the tail of the path is still exactly right and only
  the leading directories are stale, and exactly one file ends that way.
  Leading `../` counts as part of the stale prefix.
- `percent-decoding` — decoding the target yields the link that was meant,
  *and* the decoded text names a path that exists.

Both are checked by re-resolving the link pkm would write, as a path.
Decoding is not a doorway to anything looser: pkm resolves a bare name by
stem and by alias when it reads a vault, and neither is evidence for an edit.
`[[Docs/My%20Note]]` is safe; `[[My%20Note]]` reaches the same file by
filename and is only a suggestion.

Everything else is a suggestion at most, and pkm will not act on it:

- `basename` — the filename matches and the directory does not. A
  resemblance, not a path.
- An alias — a name the author gave a note is not a location.
- `path-suffix` with more than one hit — both destinations are listed and
  neither is chosen.
- A renamed concept (`[[Roam]]` beside `RoamResearch.md`) is `unresolved`.
  Do not "fix" it on pkm's behalf; ask the user.
- `[[2026-07-04]]` (`missing-date`) and `[[12]]` (`numeric-label`) name no
  note, so nothing is invented — no note, no tag.
- org links are reported with a line and never with a plan.

A link that already resolves is never a repair, including a valid
note-relative link; excluded notes and non-note leaves *are* valid
destinations, exactly as they are for the resolver.

JSON reports `kind`, `dry_run`, `applied`, `summary`
(`files`/`occurrences`/`safe`/`ambiguous`/`unresolved`) and `occurrences`.
Each occurrence adds `candidates` (`path`, `basis`, `matched`) and, when it is
safe, an `edit` with `byte_start`, `byte_end`, `before`, `after`,
`target_before`, `target_after` and `resolves_to` — enough to carry the repair
out without scanning again. `edit` is `null` for everything that is not safe.
Occurrences are ordered by source, then line, then column.

## Tasks

Plain markdown checkboxes. The Obsidian Tasks emoji convention (`📅` due,
`⏳` scheduled, `🛫` start, `✅` done, `➕` created, `🔁` recurring, `⏫🔼🔽`
priority) is read and written but never required.

Unlike links and tags, a checkbox is *not* masked out of a code fence or a
`%%comment%%` by default: a `- [ ]` inside a fenced example counts. That is
deliberate -- masking can only ever *hide* a task -- so the default errs toward
showing you too much rather than silently dropping something you meant to do.

`--prose-only` opts out, for a vault that documents its own conventions:

```bash
pkm tasks --prose-only    # skip fenced, commented and inline-code checkboxes
```

It drops the examples and keeps a real task whose text merely contains an
inline-code span. `exclude:` in `pkm.yaml` is the other answer when
the examples all live in one folder.

```bash
pkm tasks                                   # open tasks
pkm tasks --overdue
pkm tasks --available                      # start date absent or <= today
pkm tasks --due-from 2026-08-01 --due-to 2026-08-31
pkm tasks --start-on 2026-08-01
pkm tasks --scheduled-from 2026-08-01 --scheduled-to 2026-08-07
pkm tasks --status wip --tag work
pkm tasks --breadcrumbs                     # with the heading path
pkm tasks --group date                      # or --group file
pkm tasks --exclude Archive/
```

Dates must be `YYYY-MM-DD`. Relative values such as `today` or `+3d` are
rejected rather than guessed at, with exit status 2.

Tasks are selected by their full body (the `text` field of `tasks --format
json`), including any `^block-id`, which status changes keep at the end.
Completing a recurring task leaves its block ID, `🆔` and `⛔` on the
completed occurrence; the next occurrence gets none, as in the Tasks plugin.

Changing a task:

```bash
pkm tasks new "write the README" --start 2026-07-28 --due 2026-08-01
pkm tasks done "write the README"    # full task body, including any markers
pkm tasks done --file Tasks.md "first" "second" --dry-run
pkm tasks done --file Tasks.md "first" "second"
pkm tasks wip TEXT
pkm tasks cancel TEXT
pkm tasks cancel --file Tasks.md "first" "second"
pkm tasks set STATUS TEXT            # config-defined statuses too
```

Task status changes match the full checkbox body exactly by default, including
date and recurrence markers. `--partial` opts into substring matching. When an
exact match fails, the error shows up to three plausible full texts with their
file and line. An ambiguous match is refused, not guessed. Narrow it with `--file`.
Multiple texts for `done` or `cancel` require an exact vault-relative `--file`.
The command checks every match before writing the file once; a missing,
ambiguous, or repeated match leaves the file unchanged. `--dry-run` previews
the same changes without writing.

Built-in statuses: `open` `[ ]`, `wip` `[/]`, `done` `[x]`, `cancel` `[-]`.
`pkm.yaml` can override their markers or add new ones under
`tasks.statuses`. `done` stamps `✅ YYYY-MM-DD` and `cancel` stamps
`❌ YYYY-MM-DD`; `date_format: null` on a status stops it stamping anything.

## Create a note from a template

Use a template file directly, with both paths relative to the vault:

```bash
pkm new "Example project" --template Templates/Project.md --folder Projects
```

`--folder` is required for a template file. To reuse a name and default folder,
register them in `pkm.yaml`:

```yaml
templates:
  Project:
    file: Templates/Project.md
    folder: Projects
```

The file and folder are relative to the vault. `folder: .` writes at its root.
Add the template folder to `exclude` so lint does not check unexpanded templates.
Then create `Templates/Project.md`, for example:

```markdown
---
type: project
status: active
updated: {{date}}
---
# {{title}}

## Summary

## Next steps

## References
```

The template controls the note structure; `Project` is a configured name with
no built-in rules. Customize its frontmatter and headings for your vault.

```bash
pkm new "Example project" --template Project
pkm lint "Projects/Example project.md"
pkm new "Another project" --template Project --path-only
```

A registered name takes precedence over a file with the same name. `--folder`
can also override a registered template's destination. `new` writes
`Projects/Example project.md`; `.md` is always appended to the supplied title.
Slashes, control characters, empty titles, and titles or destination folders
starting with a dot are refused.
See [Template variables](#template-variables) for date, time, and title expansion.

An unknown or unreadable template, an existing destination, or a path outside
the vault fails without overwriting a note. Template and destination paths must
not contain symlinks. `--path-only` prints the vault-relative path; `--format json`
returns `path`, `title`, `template`, and `created`. Both modes create the note.
Run `lint` on the returned path to check your configured note rules.

## Template variables

`new` and `daily` use the same template expansion. Both `core` and `templater`
engines support these placeholders:

| Placeholder | Value |
|---|---|
| `{{title}}` | New note title, or the formatted daily-note filename without its extension |
| `{{date}}` | Date in `YYYY-MM-DD` format |
| `{{date:YYYY/MM/DD}}` | Date with the specified format |
| `{{time}}` | Creation time in `HH:mm` format |
| `{{time:HH:mm:ss}}` | Creation time with seconds |

`new` uses the local creation date. `daily` uses the requested note date,
even for a past or future note. Time always comes from the local clock when
the note is created. Existing daily notes are not expanded again.

Date formats replace `YYYY`, `MM`, and `DD`. Time formats replace `HH`, `mm`,
and `ss`. Other characters are preserved; this is not a full Moment.js format
engine. Empty Core formats such as `{{date:}}` remain unchanged;
Templater date helpers accept an empty format as the default.

The default `template_engine: templater` additionally supports:

| Expression | Value |
|---|---|
| `<% tp.file.title %>` | Same title as `{{title}}` |
| `<% tp.date.now() %>` | Same date as `{{date}}` |
| `<% tp.date.now("YYYY/MM/DD") %>` | Formatted date |
| `<% tp.date.now("YYYY-MM-DD", -1) %>` | Date shifted by the signed number of days |
| `<% tp.date.yesterday() %>` | Date shifted by -1 day |
| `<% tp.date.tomorrow("YYYY/MM/DD") %>` | Date shifted by +1 day, with an optional format |

Templater format arguments use double quotes. Date helpers are relative to
the requested note date for `daily`, and the creation date for `new`.
`template_engine: core` leaves all `<% ... %>` expressions unchanged.
Neither engine executes JavaScript or accepts user-defined variables.
Unsupported expressions remain in the generated note unchanged.

## Daily notes

```bash
pkm daily                # today, created from the template if missing
pkm daily yesterday
pkm daily 2026-01-15
pkm daily --path-only    # just the path, for scripting
```

Templates expand on creation: Obsidian core (`{{date}}`, `{{time}}`,
`{{title}}`) and Templater date expressions (`<% tp.date.now("YYYY-MM-DD", -1) %>`,
`<% tp.date.yesterday() %>`, `<% tp.file.title %>`).

If `daily_notes.template` names a file that cannot be read, `daily` exits
non-zero and writes nothing -- no note, and not the folder either. Report the
error rather than retrying; the fix is in `pkm.yaml` or on disk. A
vault that configured no template gets a plain `# YYYY-MM-DD`.

`pkm init` creates the template it configures, so `init` then `daily`
works in an empty directory. An existing template is never overwritten, not
even with `--force`.

## Frontmatter and tags

```bash
pkm frontmatter get FILE [KEY]
pkm frontmatter set FILE status done
pkm frontmatter delete FILE KEY
pkm tags                 # every tag with counts
pkm tags --find cli      # files carrying a tag
```

Tags are Unicode-aware and nest: `#日本語` and `#parent/child` both work.
`tags --find` matches one tag exactly; `backlinks '#tag'` covers nested tags
too and reports the line each occurrence is on.

## External references

A note can point at a value it does not contain -- an address, a key, a token
-- with an ordinary markdown link:

```markdown
[Billing address for invoices](pkm://personal/address.billing)
```

The shape is `pkm://<provider>/<locator>`. The provider is a lowercase
name the user chose, not a tool; it uses ASCII letters, digits, `_`, and `-`.
The locator is opaque but its alphabet is fixed: ASCII letters, digits, `.`,
`_`, `-`, `/`, with a maximum of 128 characters and no empty, `.`, or `..`
segments. A destination outside that grammar is silently not a reference; if
an expected link is missing from `refs`, check its spelling first.

Find them:

```bash
pkm refs FILE --format json
pkm refs --provider personal --format json
pkm refs --format paths
```

Results include `path`, `line`, `column`, `uri`, `provider`, `locator` and
`label`. References inside fenced code, inline code and `%%comments%%` are
ignored, and `pkm context FILE --format json` reports the same under
`references`. A `pkm://` link is external: it is not part of the link
graph and `rename` and `move` do not touch it.

Read one value:

```bash
pkm resolve "pkm://personal/address.billing"
pkm resolve "pkm://work/tokens/ci.deploy" --dry-run   # print the argv, run nothing
pkm resolve "pkm://work/tokens/ci.deploy" --timeout 30
```

`resolve` runs the command the user configured for that provider and prints
its stdout, with one trailing newline removed and no newline added. It exits
**2** on a malformed reference, **3** when the provider is not configured, and
**4** when the command would not run, failed, timed out, or returned nothing
usable. Values must be UTF-8 text of at most 1 MiB. It may prompt the user
interactively -- for a PIN, a passphrase or a hardware key -- so run it in the
foreground.

There is no default timeout. `--timeout SECS` bounds the whole resolve,
including the wait for provider stdout to close; a provider remains responsible
for any further processes it starts.

Provider commands live in `$XDG_CONFIG_HOME/pkm/providers.yaml`
(`~/.config/pkm/providers.yaml` by default), never in the vault:

```bash
pkm provider list
pkm provider set personal -- op read 'op://pkm/{locator}/value'
pkm provider set family -- pass show 'pkm/{locator}'
pkm provider remove personal
```

`command` is argv, not a shell line; every `{locator}` in it is replaced.
pkm is provider-agnostic and knows nothing about any password manager --
`op` above is only an example, and `pass`, `security`, `bw`, `vault` or a
local script work the same way.

Rules to follow: `pkm refs` shows only *where* a reference is, never what
it is worth -- do not claim a value from it. Never write a resolved value into
a note, a commit, a log or a file; pkm itself does not cache or store one.
Never propose putting provider commands in `pkm.yaml`; the vault is
not a place executable configuration can come from. pkm refuses a
`providers:` block there by name, so suggesting one breaks every vault
command until it is removed.

## Vault health

```bash
pkm lint Notes/One.md "Notes/Two words.md"
pkm lint --diff             # staged + unstaged + untracked notes
pkm lint --diff main        # working-tree changes against ref + untracked notes
```

Filenames are vault-relative or absolute. Explicit files and `--diff` cannot be
combined. Invalid files or refs are errors; deleted and excluded notes are not
selected, and an empty selection returns 0. JSON adds `scope.files`.
Selection limits reported source notes, while graph/duplicate checks still use
unchanged peers. Run full lint after deletions or rule changes to catch effects
on unchanged notes; `--diff` does not lint a staged-content snapshot.
Existing in-vault directories are valid link targets without requiring README.


`pkm lint` exits 0 when no issues are found and 1 when any configured
check reports an issue, including info and warning findings. Text and JSON
reports are still printed in full; JSON includes `summary.total_issues`.

`pkm.yaml` is a complete YAML document, not Markdown frontmatter. It
includes a `yaml-language-server` schema modeline when created by `pkm
init`. Use `pkm config check` for the authoritative runtime validation,
and `pkm config schema` to print the schema for an editor or CI tool.

```bash
pkm lint                 # broken links, orphans, duplicates, stubs, missing frontmatter
pkm lint --check broken-links --format json
pkm repair-links --dry-run   # which of those broken links could be repaired
```

`pkm.yaml` can scope or disable the individual `lint` rules:

```yaml
lint:
  rules:
    broken-links:
      enabled: true
      include: [Projects/]
      exclude: [Projects/archive/]
    orphans:
      enabled: false
```

With no `lint.rules` block every check is enabled. A disabled rule is omitted
from a plain `pkm lint`, while an explicit `--check RULE` runs it anyway;
the configured `include`/`exclude` path scope still applies. `include` matches
vault-relative path prefixes (empty or absent means all), and `exclude` wins.
Broken links use the source path; orphan, duplicate, empty and frontmatter
checks use the reported file path. Unknown rule names/fields/types and unknown
`--check` names are errors.

For conditional policies, add an ordered `lint.paths` list. Each entry requires
`path` (Rust regex), `where` (nonempty list of query expressions), or both.
The path matches the complete vault-relative path without implicit anchors;
all `where` expressions and the path must match. `where: ["type=manifest"]`
selects notes by metadata rather than validating that field. Combine a path-only
required-field rule with a type filter to catch missing types. A later entry wins for a check it names; an unnamed
check inherits its global rule. Checks can be boolean shorthand or blocks:

```yaml
lint:
  paths:
    - path: ^Questions/
      frontmatter:
        required: [status]
        fields:
          status:
            type: string
            enum: [open, deciding, decided, dropped]
            required_if: {field: kind, equals: decision}
    - path: ^Projects/
      broken-links:
        pattern: "^legacy/"
```

A path block enables its check unless `enabled: false` is given. The
`broken-links.pattern` applies after resolution and `ignore_links` filtering,
to normalized unresolved targets. Frontmatter fields can be typed as
`string`, `number`, `boolean`, `date`, `list` or `object`, constrained with a
scalar `enum`, and made conditional with `required_if`. Field-level failures
appear with `file` and `field` and count toward `frontmatter_errors` and
`total_issues`.
An empty YAML value (`key:` or `key: null`) counts as unset: optional fields
may remain empty, while `required` and a matching `required_if` still report it.

## Keeping pkm current

```bash
pkm self-update --check
pkm self-update
```

The updater verifies Sigstore release provenance before atomically installing
the authenticated archive through its embedded installer. `--yes` confirms a
noninteractive update, `--force` reinstalls the latest release, and `--no-skill`
preserves externally managed skills. Interactive commands notify of updates
using a six-hour background check; set `PKM_NO_UPDATE_CHECK=1` to disable it.
No update notices are written to stdout. Cache files live in
`$XDG_CACHE_HOME/pkm` or `~/.cache/pkm`.

## Notes that matter in practice

- A malformed YAML header never aborts a whole-vault scan; that note is read
  without its frontmatter.
- Links inside code fences, inline code and `%%comments%%` are not links, and
  do not appear in the graph. The same goes for tags.
- A tag is never a graph node. `orphans`, `hubs` and `broken-links` do not
  change shape because a vault uses tags, and a `#tag` is never broken.
- `pkm.yaml` can `exclude:` whole subtrees (imported archives,
  generated logs, unexpanded templates). Every command honours it, and it is
  the only way a subtree becomes invisible -- pkm has no folder names it
  treats specially.
- `pkm.yaml` is validated strictly: an unknown key, a key of the
  wrong type, or an unsupported `template_engine`/`flavor` is an error naming
  the key, not a silent default. Notes stay lenient -- a broken YAML header
  costs that note its frontmatter and nothing more.
- `pkm.yaml` can also `ignore_links:` link targets that are meant to
  stay unresolved. `lint`, `broken-links` and `query --where broken>0` stop
  reporting them. An entry matches a whole target, case-insensitively; it is
  never a substring, and a path-qualified link needs its path written out.
- `.org` files are read by a dedicated parser, including `[[id:...]]` and
  global `[[*Heading]]` links.
- `repair-links` never writes, and never guesses. A broken link it leaves
  `unresolved` is a question for the user, not an invitation to invent a note
  or demote it to a tag. `demote` is for a label that was never a reference;
  it is not link repair.

### Required headings

Require section titles for selected paths with `lint.paths`:

```yaml
lint:
  paths:
    - path: '^Wiki/acquaintances/[^/]+\.md$'
      headings:
        required: [Bio, Relationship, Interactions, Sources]
```

Run `pkm lint --check headings --format json`. Each missing title produces
an issue with `type: headings`, `file`, `heading`, `detail`, and `severity`;
`summary.missing_headings` counts missing titles. Text output lists the file and
missing title. Like other lint checks, any finding produces exit code 1;
automation can also inspect `summary.total_issues`.

Titles use the same normalization as heading anchors (case-insensitive, surrounding
whitespace and trailing ATX decoration ignored; inline markup remains literal). ATX and Setext Markdown headings and Org headings count;
frontmatter, closed code blocks, and comments do not. Level, order, uniqueness,
and section contents are not constrained. Duplicate normalized requirements
produce only one issue. An empty `required` list imposes no constraint. No
required headings are assumed for unconfigured paths.

`headings: false` disables a matching path; `true` enables it without requirements.
Global `lint.rules.headings` supports enabled/include/exclude, and later matching
path rules replace earlier heading policies in full (including global scope),
following the other lint checks. Top-level `exclude` always applies.
