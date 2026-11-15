# knapper frontmatter

```text
Read and write frontmatter

Usage: knapper frontmatter [OPTIONS] <COMMAND>

Commands:
  get     Get frontmatter value(s)
  set     Set frontmatter value
  delete  Delete frontmatter key
  help    Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  Path to config file
  -v, --vault <VAULT>    Path to vault (overrides config)
  -h, --help             Print help
```

## knapper frontmatter get

```text
Get frontmatter value(s)

Usage: knapper frontmatter get [OPTIONS] <FILE> [KEY]

Arguments:
  <FILE>
  [KEY]

Options:
  -c, --config <CONFIG>  Path to config file
  -f, --format <FORMAT>  [default: text]
  -v, --vault <VAULT>    Path to vault (overrides config)
  -h, --help             Print help
```

## knapper frontmatter set

```text
Set frontmatter value

Usage: knapper frontmatter set [OPTIONS] <FILE> <KEY> <VALUE>

Arguments:
  <FILE>
  <KEY>
  <VALUE>

Options:
  -c, --config <CONFIG>  Path to config file
  -v, --vault <VAULT>    Path to vault (overrides config)
  -h, --help             Print help
```

## knapper frontmatter delete

```text
Delete frontmatter key

Usage: knapper frontmatter delete [OPTIONS] <FILE> <KEY>

Arguments:
  <FILE>
  <KEY>

Options:
  -c, --config <CONFIG>  Path to config file
  -v, --vault <VAULT>    Path to vault (overrides config)
  -h, --help             Print help
```
