# Lint rules

`pkm lint` checks the vault for unreadable notes, broken links, orphans, duplicate names, empty notes, frontmatter and required headings. Rules are configured in `pkm.yaml` and can be scoped by path or by metadata.

## Selecting files for lint

```bash
pkm lint Notes/One.md "Notes/Two words.md"
pkm lint --diff                 # staged + unstaged + untracked notes
pkm lint --diff main            # working-tree changes against this ref + untracked notes
pkm lint Notes/One.md --check headings --format json
```

File arguments are vault-relative or absolute and may omit the note extension.
Missing, excluded and out-of-vault files are rejected. File arguments and `--diff`
cannot be combined. Git selection handles spaces, Unicode, renames and an unborn
repository; deleted files, ignored untracked files and non-notes are not selected.
A missing Git repository or invalid ref is an error, never a whole-vault fallback.
An empty selection reports zero issues and exits 0. JSON includes `scope.files`
for either selection mode; plain `lint` still checks the whole configured vault.

Selection limits the source files reported, not the link-resolution universe:
links may point to unchanged files, incoming links still prevent false orphans,
and duplicate checks compare a selected note against unchanged peers in rule scope.
Local checks only read selected note bodies (plus peers when a duplicate policy
needs them). Graph checks and graph-field filters still read the full vault.
Run a full lint after deleting notes or changing rules to catch effects on
unchanged notes. `--diff` reads the working tree, not a staged-content snapshot.

## Configuring lint rules

Heading links accept visible titles and common Markdown slugs (`#quick-start`
for `Quick Start`), including numbered duplicate slugs. Required-heading policies
still compare heading titles, not slugs.

`lint.rules` controls the checks reported by `pkm lint`:
`unreadable`, `broken-links`, `orphans`, `duplicates`, `empty`, `frontmatter`, `headings` and `single-entry`. Every rule
is enabled with no configuration, preserving the default all-checks report.
Set `enabled: false` to omit a rule from a plain `pkm lint`; an explicit
`pkm lint --check RULE` always runs that rule. `--check` may be repeated.

`include` and `exclude` are optional vault-relative path-prefix lists. A file
must be under at least one `include` prefix (an absent or empty list means all
files), and `exclude` wins when both match. Broken links are filtered by their
source file. The other file checks use the file being reported. Duplicate
groups are kept only when at least two paths remain in scope, and only those
paths are shown. Unknown rule names, fields and value types are configuration
errors, as are unknown `--check` names.

For finer-grained policies, `lint.paths` is an ordered list. Each entry
requires `path`, `where`, or both. `path` is a Rust regex matched against the
complete vault-relative path without adding anchors. `where` is a nonempty list
of the same expressions accepted by `query --where`: frontmatter, inline fields,
tags and computed fields are available. All expressions and the path must match. A path entry may set any check to a boolean shorthand
or a block; when several entries match, the last entry that names that check
wins, while an unnamed check inherits the global rule. A block enables its
check by default, so a path can opt into a globally disabled check. The
`broken-links` block accepts a `pattern` regex for normalized unresolved link
targets. `frontmatter` accepts `required` keys and `fields` rules with
`type` (`string`, `number`, `boolean`, `date`, `list`, `object`), scalar
`enum`, and `required_if: {field, equals}`. Field errors are reported with the
file and field and are included in `frontmatter_errors` and `total_issues`.
An empty YAML value (`key:` or `key: null`) counts as unset: optional fields
may remain empty, while `required` and a matching `required_if` still report it.

For example, apply a policy by note type rather than filename:

```yaml
lint:
  paths:
    - path: '^Logs/'
      where: ['type=work-log-manifest', '!archived']
      frontmatter:
        required: [context, status]
      headings:
        required: [Result]
```

The filter selects notes; it does not require the filter field to exist.
A separate path-only rule can require `type` so missing or mistyped types do
not silently fall outside the intended policy.

Existing in-vault directory links, such as `[docs](../docs/)`, are valid even
without a README. Directories are not note nodes or duplicate/orphan candidates.
Directory lookup never falls back to an unrelated basename, follows a directory
symlink, or resolves a path outside the vault.

Missing headings and block IDs are part of `broken-links`, so they inherit its
global and path-specific `include`, `exclude`, `enabled`, and `pattern`
settings. For an anchor failure, `pattern` sees the complete lint target after
path normalization, such as `Guide#Install` or `#^build-step`; the anchor's
written spelling is retained.

Every setting is checked when it is read. An unknown key, a key of the wrong
type, or a `template_engine` or `flavor` pkm does not implement is an
error naming the key, rather than a default quietly taking over — a config
typo otherwise changes what every command does and says nothing. This strictness
is the config's alone: an ordinary note with a broken YAML header is still read
without its frontmatter rather than aborting a scan.

No folder name is special. `Templates/` is in the generated `exclude` because
the daily-note template lives there and an unexpanded template is not one of
your notes; put templates elsewhere and the `exclude` entry moves with them.
`daily_notes.template` is read by path, so excluding its folder does not stop
`pkm daily`. If that template is configured and cannot be read, `pkm
daily` fails and writes nothing at all, rather than inventing a note body —
which is why `pkm init` creates the template it configures rather than
leaving a fresh vault one command short of working.

## Notes that cannot be read

Notes are UTF-8. A note in another encoding, such as Shift_JIS or UTF-16, is
left out of every command: its tasks, tags and links are not counted, and no
command edits it. `unreadable` is the rule that says so:

```json
{
  "type": "unreadable",
  "file": "Imports/old-memo.md",
  "detail": "Not valid UTF-8 (first invalid byte at offset 418)",
  "severity": "error"
}
```

A note the operating system refuses to open is reported the same way, with the
system's reason as `detail`. pkm does not guess or convert encodings. Save the
note as UTF-8, for example with `iconv -f SHIFT_JIS -t UTF-8`.

A UTF-8 byte order mark at the start of a note is not an error. It is ignored
when the note is read, so frontmatter and a first-line heading work as they do
without it, and it is kept when pkm rewrites the note. Notes pkm creates
do not start with one.

## Links that are meant to stay unresolved

A vault that came from somewhere else carries links that will never resolve
and are not mistakes — a tag page from an import, a name a generator emitted.
Thousands of them in a lint report bury the broken links that are real, so the
vault names them once:

```yaml
ignore_links:
  - Daily Tasks
  - Habits
  - Archive/Old Index
```

They then go unreported by `pkm lint`, `pkm broken-links`,
`pkm repair-links` and `query --where 'broken>0'` alike — all four resolve
targets the same way and read the same ignore list, so a link is ignored by
all of them or by none.

The matching is deliberately narrow, because an ignore that reached too far
would hide a real mistake:

- An entry matches a **whole link target**, never a substring. `Daily Tasks`
  says nothing about `[[Daily Tasks Archive]]` or `[[Sub/Daily Tasks]]`.
- Comparison is **case-insensitive**, matching how pkm resolves links.
- The target compared is the one pkm resolves, so `[[Daily Tasks]]`,
  `[[Daily Tasks#2026]]`, `[[Daily Tasks|things to do]]` and
  `[Daily Tasks](Daily%20Tasks.md)` are all the same entry.
- An entry may be written the way the link is written in a note:
  `[[Habits]]`, `Habits.md` and `Habits` are equivalent.
- A **path-qualified** link is its own target. To ignore `[[Archive/Old
  Index]]`, write `Archive/Old Index`.
- `exclude` and `ignore_links` answer different questions: `exclude` hides
  notes from every command, `ignore_links` only stops a link target from being
  called broken.

## Required headings

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
missing title. Like other lint checks, the command exits 1 when any findings
are reported and 0 when `summary.total_issues` is zero. Both text and JSON
output retain the complete report, including findings with info or warning severity.

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

## Broken-link inspection scope

`pkm broken-links` respects the broken-links policies used by ordinary
`pkm lint`, including enablement, include/exclude, ordered path and metadata
rules, and target patterns. Its output retains detailed occurrence locations.
Use `pkm broken-links --all` to bypass those lint policies for investigation;
top-level `exclude` and `ignore_links` still apply. `repair-links` retains its
unfiltered diagnostic scope.

## Single-entry layout

A single-entry layout gives each unit exactly one entry note. A standalone note
is its own entry; a child folder uses a configured main filename. This is useful
for projects, courses or other collections with supporting notes.

```yaml
lint:
  single_entry:
    - path: Projects
      main: README.md
      exclude: [assets]
      entry:
        type: project
  paths:
    - path: '^Projects/'
      where: ['type=project']
      frontmatter:
        required: [type, status, updated]
      headings:
        required: [Summary, Next steps, References]
```

`path` is a vault-relative directory, not a regex. Each direct note and child
folder is one unit. `main` defaults to `README.md`; set it to a single note
filename such as `index.md` or `overview.org`. The root's own main file is the
collection index and is not a unit. Hidden entries are ignored. Non-note files
at the collection root are not units.

`exclude` contains path prefixes relative to `path`, including nested paths.
Top-level vault exclusions also apply. Every non-excluded child folder requires
its main file, even if empty. Layout roots and entries must not be symlinks;
the checker never follows directory symlinks.

Optional `entry` fields are exact, non-null scalar frontmatter values. They are
required on each entry and forbidden as a complete matching set on supporting
notes anywhere inside a unit. With no `entry` fields, the check validates file
placement only. Use ordinary frontmatter and heading rules for additional entry
requirements; metadata missing from an entry is caught by the layout check
before a type-based policy could omit it.

The check also reports a standalone note and a same-named child folder, comparing
names case-insensitively after Unicode NFC normalization. It reports issues;
it does not move files or change metadata.

Run `pkm lint --check single-entry --format json`. Issues have `type:
"single-entry"`, a vault-relative `file`, `code`, `detail` and `severity`.
`summary.single_entry` counts them. The codes are `missing-main`,
`entry-metadata`, `extra-entry`, `duplicate-unit`, `invalid-root` and `symlink`.
Use `lint.rules.single-entry` to disable or scope reporting, or ordinary
`lint.paths` overrides. The check is inert unless `lint.single_entry` is configured.

When selecting files, including with `--diff`, layout inspection expands to the
selected files' units so that a changed supporting note can reveal a missing
entry. JSON `scope.files` still lists the originally selected files; layout
issues may point at peers or an expected main file that does not exist. As with
other checks, deleted notes are not selected by `--diff`: run a full lint after
deleting an entry or changing layout configuration.
