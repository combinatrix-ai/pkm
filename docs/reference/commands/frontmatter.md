# pkm frontmatter

```text
Read and write frontmatter

Usage: pkm frontmatter [OPTIONS] <COMMAND>

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

## pkm frontmatter get

```text
Get frontmatter value(s)

Usage: pkm frontmatter get [OPTIONS] <FILE> [KEY]

Arguments:
  <FILE>
  [KEY]

Options:
  -c, --config <CONFIG>  Path to config file
  -f, --format <FORMAT>  [default: text]
  -v, --vault <VAULT>    Path to vault (overrides config)
  -h, --help             Print help
```

## pkm frontmatter set

```text
Set frontmatter value

Usage: pkm frontmatter set [OPTIONS] <FILE> <KEY> <VALUE>

Arguments:
  <FILE>
  <KEY>
  <VALUE>

Options:
  -c, --config <CONFIG>  Path to config file
  -v, --vault <VAULT>    Path to vault (overrides config)
  -h, --help             Print help
```

## pkm frontmatter delete

```text
Delete frontmatter key

Usage: pkm frontmatter delete [OPTIONS] <FILE> <KEY>

Arguments:
  <FILE>
  <KEY>

Options:
  -c, --config <CONFIG>  Path to config file
  -v, --vault <VAULT>    Path to vault (overrides config)
  -h, --help             Print help
```
