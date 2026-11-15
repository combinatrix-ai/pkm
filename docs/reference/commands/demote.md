# knapper demote

```text
Demote a wikilink target to a tag.

[[X]] is a hard note reference: a missing target is a broken link. #X is a soft
topic reference: it labels a note and promises no note exists. This rewrites the
exact form -- [[X]] becomes #X -- and reports every occurrence it will not touch,
such as [[X|alias]], [[X#heading]], ![[X]] and [[folder/X]].

Usage: knapper demote [OPTIONS] <TARGET>

Arguments:
  <TARGET>
          A wikilink target: X or [[X]]

Options:
  -c, --config <CONFIG>
          Path to config file

      --tag <TAG>
          Write this tag instead of the target itself

      --dry-run
          

  -v, --vault <VAULT>
          Path to vault (overrides config)

      --allow-existing-note
          Demote even though the target names a note that exists

  -f, --format <FORMAT>
          [default: text]

  -h, --help
          Print help (see a summary with '-h')
```
