# Start with a folder of notes

<div v-pre>

knapper works directly on Markdown files. It needs no desktop app, server, or index.

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/combinatrix-ai/knapper/v0.1.0/install.sh | sh
```

The installer verifies archive checksums and installs to `~/.local/bin`.
See [installation details](../reference/cli#install) for supported architectures,
provenance verification, and installation options.

## Initialize

```sh
cd ~/your-notes
knapper init
```

This writes `knapper.yaml` and a daily-note template. Existing templates are preserved.

## Create a note

Save a template as `Templates/Project.md`:

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

```sh
knapper new "Field notes" --template Templates/Project.md --folder Projects
knapper lint "Projects/Field notes.md"
```

`new` refuses to overwrite existing files. You can also register a template name
and default folder in `knapper.yaml`.
See [templates](../reference/cli#create-a-note-from-a-template) and
[template variables](../reference/cli#template-variables).

## Follow the connections

```sh
knapper backlinks "Projects/Field notes.md"
knapper query --where 'status=active' --format json
knapper tasks --overdue
knapper rename "Field notes" "Research notes" --dry-run
```

Use `rg` for full-text searches, and knapper for links, metadata, tasks, and
structural edits. Add `--format json` when another program consumes the output.

## Update

```sh
knapper self-update
```

The updater verifies release provenance. Add `--no-skill` if you manage the
embedded agent skill separately.

</div>
