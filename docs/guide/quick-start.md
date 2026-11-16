# Start with a folder of notes

knapper works directly on Markdown files. It needs no desktop app, server, or index.

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/combinatrix-ai/knapper/v0.1.0/install.sh | sh
```

The installer verifies archive checksums and installs to `~/.local/bin`.
See [Install](install.md) for supported architectures,
provenance verification, and installation options.

## Initialize

```sh
cd ~/your-notes
knapper init
```

This writes `knapper.yaml` and the `Templates/daily.md` it points at, so
`knapper daily` works straight afterwards. A template you already have is never
overwritten, `--force` included: that flag replaces knapper's config, not your
writing.

`knapper init` is one-time. After that, knapper walks up from wherever you are
to find the nearest `knapper.yaml`, so commands work from any subdirectory.

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
See [Templates and daily notes](templates.md).

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

## Hand it to your agent

```sh
knapper skill --install
```

The installer already registers the embedded skill with Claude Code and Codex
when it finds them. See [Working with agents](agents.md).

## Where next

- [Querying notes](querying.md)
- [Renames that don't break anything](refactoring.md)
- [Tasks](tasks.md)
- [Lint rules](lint.md)
- [CLI reference](../reference/README.md)
