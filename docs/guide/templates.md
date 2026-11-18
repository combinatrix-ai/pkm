# Templates and daily notes

Create notes from templates, expand dates and titles, and keep a daily note.

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

`--path-only` changes the output, not the operation: it still creates the
note if missing. JSON output reports `path`, `date`, and `created`.

```bash
pkm daily              # today (created from template if missing)
pkm daily yesterday
pkm daily 2026-01-15
```

Templates expand on creation — both Obsidian Core Templates (`{{date}}`,
`{{time}}`, `{{title}}`) and Templater date expressions:

```markdown
<!-- template -->
<< [[Daily/<% tp.date.now("YYYY-MM-DD", -1) %>]] | [[Daily/<% tp.date.now("YYYY-MM-DD", 1) %>]] >>

<!-- created file, 2026-07-28 -->
<< [[Daily/2026-07-27]] | [[Daily/2026-07-29]] >>
```

If `daily_notes.template` names a file pkm cannot read, `pkm daily`
fails and writes nothing — no note, not even the folder that would have held
it. A vault that names no template at all gets a plain `# YYYY-MM-DD`, because
that is all it asked for.
