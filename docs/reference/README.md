# CLI reference

Every public command, with its arguments and options. For how the commands fit together, start with the [guides](../README.md).

## Invocation and output

```text
pkm [--config PATH] [--vault PATH] COMMAND [OPTIONS]
```

`-c/--config` selects a config file. `-v/--vault` overrides the vault path.
Both options are global and may be passed with any subcommand. Without an
explicit config, pkm searches upward for `pkm.yaml`.

Commands with `-f/--format` default to text. Use `--format json` for scripts;
list-producing commands also support paths where described below. Output
formats are command-specific: there is no global `--format` option.
Quote predicates such as `'inlinks>3'` so the shell does not interpret `>`
as redirection, and quote tag arguments such as `'#research'`.

Use `pkm --help`, `pkm COMMAND --help`, or
`pkm COMMAND SUBCOMMAND --help` for the installed version's syntax.
The [command list](#commands) below links every public command and
subcommand. The hidden background update helper is internal.

Ordinary successful commands exit 0; operational errors exit 1, and argument
parsing errors exit 2. `lint` also exits 1 when it reports any findings,
including info or warning severity. `resolve` has the additional exit codes
listed in [Resolving](../guide/external-references.md#resolving).

## Commands

| Command | What it does |
|---------|--------------|
| [`pkm init`](commands/init.md) | Write `pkm.yaml`, and the daily template it names |
| [`pkm config check`](commands/config.md) | Validate the config without scanning the vault |
| [`pkm config schema`](commands/config.md) | Print the JSON Schema for YAML-aware editors |
| [`pkm query`](commands/query.md) | Filter notes by frontmatter, inline fields and link counts |
| [`pkm fields`](commands/fields.md) | List what `query` can filter on |
| [`pkm lint`](commands/lint.md) | Vault health: `unreadable`, `broken-links`, `orphans`, `duplicates`, `empty`, `frontmatter`, `headings`, `single-entry` |
| [`pkm backlinks FILE`](commands/backlinks.md) | Incoming links to a file, or the notes carrying a `'#tag'` |
| [`pkm links FILE`](commands/links.md) | Outgoing links from a file |
| [`pkm orphans`](commands/orphans.md) | Notes no other note links to |
| [`pkm hubs`](commands/hubs.md) | Most-linked-to notes |
| [`pkm broken-links`](commands/broken-links.md) | Links to missing notes, headings or block IDs, one record per occurrence |
| [`pkm repair-links --dry-run`](commands/repair-links.md) | Plan repairs for broken links; never writes |
| [`pkm rename OLD NEW`](commands/rename.md) | Rename a note and update all links |
| [`pkm move SRC DEST`](commands/move.md) | Move a note or a directory and update all links |
| [`pkm demote TARGET`](commands/demote.md) | Rewrite the exact `[[TARGET]]` into `#TARGET` |
| [`pkm context FILE`](commands/context.md) | Aggregated context; add `--line N` for focused Markdown context |
| [`pkm tasks`](commands/tasks.md) | Find and filter `- [ ]` tasks |
| [`pkm tasks new / done / wip / cancel / set`](commands/tasks.md) | Create a task or change its status |
| [`pkm new TITLE --template NAME_OR_FILE`](commands/new.md) | Create a note from a template file or configured name |
| [`pkm daily [DATE]`](commands/daily.md) | Create or get a daily note |
| [`pkm frontmatter get / set / delete`](commands/frontmatter.md) | Read and write YAML frontmatter |
| [`pkm tags`](commands/tags.md) | List tags, or find files by tag with `--find` |
| [`pkm refs [FILE]`](commands/refs.md) | Find `pkm://` references, optionally for one provider |
| [`pkm resolve REF`](commands/resolve.md) | Read one reference's value through its provider's command |
| [`pkm provider list / set / remove`](commands/provider.md) | Configure those commands, outside the vault |
| [`pkm skill`](commands/skill.md) | Print the embedded agent skill, or `--install` it |
| [`pkm self-update`](commands/self-update.md) | Update a release binary; other installs show how to update |

Query commands take `-f/--format`. Each command page shows its `--help`
output, captured from the current source build. When changing CLI arguments,
refresh the affected page with the matching `--help` command. Global options
are repeated so each page stands alone.

## pkm

```text
pkm - a headless CLI for markdown knowledge bases.

Reads and writes markdown files directly; nothing needs to be running.
Resolves both [[wikilinks]] and [inline](links.md).

Usage: pkm [OPTIONS] <COMMAND>

Commands:
  links         List outgoing links from a file
  backlinks     List incoming links to a file, or the notes carrying a '#tag'
  orphans       Find orphan notes (not linked by any other note)
  hubs          Find hub notes (most linked-to notes)
  broken-links  Find links to missing notes, headings, or block IDs
  repair-links  Plan repairs for broken links. Reports only; writes nothing
  init          Initialize a new pkm config file
  config        Inspect or print the pkm YAML configuration schema
  skill         Print the embedded agent skill, or register it with the agents here
  self-update   Update a release binary to the newest verified release; other installs show how to update
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
  refs          List external references (pkm://provider/locator) in the vault
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
