# pkm lint

```text
Check vault health (exit 1 when any issues are found)

Usage: pkm lint [OPTIONS] [FILE]...

Arguments:
  [FILE]...  Vault-relative or absolute note filenames (repeatable)

Options:
  -c, --config <CONFIG>  Path to config file
      --check <RULE>     Run one configured lint rule (repeatable) [possible values: broken-links, orphans, duplicates, empty, frontmatter, headings]
      --diff [<REF>]     Only changed notes: staged, unstaged and untracked; optionally compare to REF
  -v, --vault <VAULT>    Path to vault (overrides config)
  -f, --format <FORMAT>  [default: text]
  -h, --help             Print help
```
