# Configuration

`knapper init` writes `knapper.yaml`, a plain YAML file. It includes a
`yaml-language-server` schema modeline, so editors that support the YAML
Language Server can offer completion, hover documentation and diagnostics.
Only `knapper.yaml` is discovered; the former Markdown-frontmatter
`knapper.config.md` is not a fallback.
The defaults assume a conventional layout:

```yaml
# yaml-language-server: $schema=https://raw.githubusercontent.com/combinatrix-ai/knapper/main/schema/knapper.schema.json
vault_path: .
template_engine: templater   # or "core"
exclude:
  - Templates/               # unexpanded templates are not notes
lint:
  rules:
    broken-links:
      enabled: true
      # include: [Projects/]  # optional path-prefix scope
      # exclude: [Projects/archive/]
    # orphans:
    #   enabled: false
    # duplicates:
    #   include: [Notes/]
  # paths:
  #   - path: ^Questions/
  #     frontmatter:
  #       required: [status]
  #       fields:
  #         status:
  #           type: string
  #           enum: [open, deciding, decided, dropped]
daily_notes:
  folder: Daily
  template: Templates/daily.md
  format: YYYY-MM-DD
tasks:
  default_file: daily        # where `tasks new` writes: daily | inbox | a path
```

The schema is also available locally with `knapper config schema`. `knapper
config check` is the authoritative runtime validator: it catches unknown keys,
wrong types, unsupported values and semantic errors such as ambiguous task
status markers without scanning any notes. Nothing in the config is
Obsidian-specific.

The complete accepted configuration keys, types and defaults are defined
in the [JSON Schema](../../schema/knapper.schema.json). The
[starter configuration](../../src/default_config.yaml) includes task status,
created-date and template settings. Validate changes with `knapper config check`.

Lint policies have their own page: [Lint rules](lint.md). Templates and daily
notes are covered in [Templates and daily notes](templates.md), and task
statuses in [Tasks](tasks.md).
