# Lint rules

`knapper lint` checks the vault for broken links, orphans, duplicate names, empty notes, frontmatter and required headings. Rules are configured in `knapper.yaml` and can be scoped by path or by metadata.

## Selecting files for lint

```bash
knapper lint Notes/One.md "Notes/Two words.md"
knapper lint --diff                 # staged + unstaged + untracked notes
knapper lint --diff main            # working-tree changes against this ref + untracked notes
knapper lint Notes/One.md --check headings --format json
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

`lint.rules` controls the six checks reported by `knapper lint`:
`broken-links`, `orphans`, `duplicates`, `empty`, `frontmatter` and `headings`. Every rule
is enabled with no configuration, preserving the default all-checks report.
Set `enabled: false` to omit a rule from a plain `knapper lint`; an explicit
`knapper lint --check RULE` always runs that rule. `--check` may be repeated.

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
type, or a `template_engine` or `flavor` knapper does not implement is an
error naming the key, rather than a default quietly taking over — a config
typo otherwise changes what every command does and says nothing. This strictness
is the config's alone: an ordinary note with a broken YAML header is still read
without its frontmatter rather than aborting a scan.

No folder name is special. `Templates/` is in the generated `exclude` because
the daily-note template lives there and an unexpanded template is not one of
your notes; put templates elsewhere and the `exclude` entry moves with them.
`daily_notes.template` is read by path, so excluding its folder does not stop
`knapper daily`. If that template is configured and cannot be read, `knapper
daily` fails and writes nothing at all, rather than inventing a note body —
which is why `knapper init` creates the template it configures rather than
leaving a fresh vault one command short of working.

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

They then go unreported by `knapper lint`, `knapper broken-links`,
`knapper repair-links` and `query --where 'broken>0'` alike — all four resolve
targets the same way and read the same ignore list, so a link is ignored by
all of them or by none.

The matching is deliberately narrow, because an ignore that reached too far
would hide a real mistake:

- An entry matches a **whole link target**, never a substring. `Daily Tasks`
  says nothing about `[[Daily Tasks Archive]]` or `[[Sub/Daily Tasks]]`.
- Comparison is **case-insensitive**, matching how knapper resolves links.
- The target compared is the one knapper resolves, so `[[Daily Tasks]]`,
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

Run `knapper lint --check headings --format json`. Each missing title produces
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

`knapper broken-links` respects the broken-links policies used by ordinary
`knapper lint`, including enablement, include/exclude, ordered path and metadata
rules, and target patterns. Its output retains detailed occurrence locations.
Use `knapper broken-links --all` to bypass those lint policies for investigation;
top-level `exclude` and `ignore_links` still apply. `repair-links` retains its
unfiltered diagnostic scope.
