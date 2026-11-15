# CLI reference

Every public command, with its arguments and options. For how the commands fit together, start with the [guides](../README.md).

## Invocation and output

```text
knapper [--config PATH] [--vault PATH] COMMAND [OPTIONS]
```

`-c/--config` selects a config file. `-v/--vault` overrides the vault path.
Both options are global and may be passed with any subcommand. Without an
explicit config, knapper searches upward for `knapper.yaml`.

Commands with `-f/--format` default to text. Use `--format json` for scripts;
list-producing commands also support paths where described below. Output
formats are command-specific: there is no global `--format` option.
Quote predicates such as `'inlinks>3'` so the shell does not interpret `>`
as redirection, and quote tag arguments such as `'#research'`.

Use `knapper --help`, `knapper COMMAND --help`, or
`knapper COMMAND SUBCOMMAND --help` for the installed version's syntax.
The [command list](#commands) below links every public command and
subcommand. The hidden background update helper is internal.

Ordinary successful commands exit 0; operational errors exit 1, and argument
parsing errors exit 2. `lint` also exits 1 when it reports any findings,
including info or warning severity. `resolve` has the additional exit codes
listed in [Resolving](../guide/external-references.md#resolving).

## Commands

| Command | What it does |
|---------|--------------|
| [`knapper init`](commands/init.md) | Write `knapper.yaml`, and the daily template it names |
| [`knapper config check`](commands/config.md) | Validate the config without scanning the vault |
| [`knapper config schema`](commands/config.md) | Print the JSON Schema for YAML-aware editors |
| [`knapper query`](commands/query.md) | Filter notes by frontmatter, inline fields and link counts |
| [`knapper fields`](commands/fields.md) | List what `query` can filter on |
| [`knapper lint`](commands/lint.md) | Vault health: `broken-links`, `orphans`, `duplicates`, `empty`, `frontmatter`, `headings` |
| [`knapper backlinks FILE`](commands/backlinks.md) | Incoming links to a file, or the notes carrying a `'#tag'` |
| [`knapper links FILE`](commands/links.md) | Outgoing links from a file |
| [`knapper orphans`](commands/orphans.md) | Notes no other note links to |
| [`knapper hubs`](commands/hubs.md) | Most-linked-to notes |
| [`knapper broken-links`](commands/broken-links.md) | Links to missing notes, headings or block IDs, one record per occurrence |
| [`knapper repair-links --dry-run`](commands/repair-links.md) | Plan repairs for broken links; never writes |
| [`knapper rename OLD NEW`](commands/rename.md) | Rename a note and update all links |
| [`knapper move SRC DEST`](commands/move.md) | Move a note or a directory and update all links |
| [`knapper demote TARGET`](commands/demote.md) | Rewrite the exact `[[TARGET]]` into `#TARGET` |
| [`knapper context FILE`](commands/context.md) | Aggregated context; add `--line N` for focused Markdown context |
| [`knapper tasks`](commands/tasks.md) | Find and filter `- [ ]` tasks |
| [`knapper tasks new / done / wip / cancel / set`](commands/tasks.md) | Create a task or change its status |
| [`knapper new TITLE --template NAME_OR_FILE`](commands/new.md) | Create a note from a template file or configured name |
| [`knapper daily [DATE]`](commands/daily.md) | Create or get a daily note |
| [`knapper frontmatter get / set / delete`](commands/frontmatter.md) | Read and write YAML frontmatter |
| [`knapper tags`](commands/tags.md) | List tags, or find files by tag with `--find` |
| [`knapper refs [FILE]`](commands/refs.md) | Find `knapper://` references, optionally for one provider |
| [`knapper resolve REF`](commands/resolve.md) | Read one reference's value through its provider's command |
| [`knapper provider list / set / remove`](commands/provider.md) | Configure those commands, outside the vault |
| [`knapper skill`](commands/skill.md) | Print the embedded agent skill, or `--install` it |
| [`knapper self-update`](commands/self-update.md) | Replace this binary with the newest release |

Query commands take `-f/--format`. Each command page shows its `--help`
output, captured from the current source build. When changing CLI arguments,
refresh the affected page with the matching `--help` command. Global options
are repeated so each page stands alone.

## knapper

```text
knapper - a headless CLI for markdown knowledge bases.

Reads and writes markdown files directly; nothing needs to be running.
Resolves both [[wikilinks]] and [inline](links.md).

Usage: knapper [OPTIONS] <COMMAND>

Commands:
  links         List outgoing links from a file
  backlinks     List incoming links to a file, or the notes carrying a '#tag'
  orphans       Find orphan notes (not linked by any other note)
  hubs          Find hub notes (most linked-to notes)
  broken-links  Find links to missing notes, headings, or block IDs
  repair-links  Plan repairs for broken links. Reports only; writes nothing
  init          Initialize a new knapper config file
  config        Inspect or print the knapper YAML configuration schema
  skill         Print the embedded agent skill, or register it with the agents here
  self-update   Verify release provenance and replace this binary with the newest release
  context       Get comprehensive context for a file or a '#tag' (for LLMs)
  frontmatter   Read and write frontmatter
  lint          Check vault health (exit 1 when any issues are found)
  new           Create a note from a configured template or a template file
  daily         Create or get daily note
  rename        Rename a note and update all links to it
  demote        Rewrite a hard [[target]] into a soft #tag
  query         Filter notes by frontmatter, inline fields, and link counts
  fields        List what query can filter on, computed and declared
  move          Move a note or a directory to a different folder and update all links
  tasks         Find and filter tasks (- [ ] items) in the vault
  tags          List tags
  refs          List external references (knapper://provider/locator) in the vault
  resolve       Read the value behind one reference, through its provider's command
  provider      Configure the commands that resolve references
  help          Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>
          Path to config file

  -v, --vault <VAULT>
          Path to vault (overrides config)

  -h, --help
          Print help (see a summary with '-h')

  -V, --version
          Print version
```
