# Start with a folder of notes

pkm works directly on Markdown files. It needs no desktop app, server, or index.

## Install

```sh
curl -fsSL https://raw.githubusercontent.com/combinatrix-ai/pkm/v0.1.0/install.sh | sh
```

The installer verifies archive checksums and installs to `~/.local/bin`.
See [Install](install.md) for supported architectures,
provenance verification, and installation options.

## Initialize

```sh
cd ~/your-notes
pkm init
```

This writes `pkm.yaml` and the `Templates/daily.md` it points at, so
`pkm daily` works straight afterwards. A template you already have is never
overwritten, `--force` included: that flag replaces pkm's config, not your
writing.

`pkm init` is one-time. After that, pkm walks up from wherever you are
to find the nearest `pkm.yaml`, so commands work from any subdirectory.

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
pkm new "Field notes" --template Templates/Project.md --folder Projects
pkm lint "Projects/Field notes.md"
```

`new` refuses to overwrite existing files. You can also register a template name
and default folder in `pkm.yaml`.
See [Templates and daily notes](templates.md).

## Follow the connections

```sh
pkm backlinks "Projects/Field notes.md"
pkm query --where 'status=active' --format json
pkm tasks --overdue
pkm rename "Field notes" "Research notes" --dry-run
```

Use `rg` for full-text searches, and pkm for links, metadata, tasks, and
structural edits. Add `--format json` when another program consumes the output.

## Update

```sh
pkm self-update
```

The updater verifies release provenance. Add `--no-skill` if you manage the
embedded agent skill separately.

## Hand it to your agent

```sh
pkm skill --install
```

The installer already registers the embedded skill with Claude Code and Codex
when it finds them. See [Working with agents](agents.md).

## Where next

- [Querying notes](querying.md)
- [Renames that don't break anything](refactoring.md)
- [Tasks](tasks.md)
- [Lint rules](lint.md)
- [CLI reference](../reference/README.md)
