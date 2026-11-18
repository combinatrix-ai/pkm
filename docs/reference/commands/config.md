# pkm config

```text
Inspect or print the pkm YAML configuration schema

Usage: pkm config [OPTIONS] <COMMAND>

Commands:
  check   Validate the discovered config without scanning the vault
  schema  Print the JSON Schema used by YAML-aware editors
  help    Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  Path to config file
  -v, --vault <VAULT>    Path to vault (overrides config)
  -h, --help             Print help
```

## pkm config check

```text
Validate the discovered config without scanning the vault

Usage: pkm config check [OPTIONS]

Options:
  -c, --config <CONFIG>  Path to config file
  -f, --format <FORMAT>  [default: text]
  -v, --vault <VAULT>    Path to vault (overrides config)
  -h, --help             Print help
```

## pkm config schema

```text
Print the JSON Schema used by YAML-aware editors

Usage: pkm config schema [OPTIONS]

Options:
  -c, --config <CONFIG>  Path to config file
  -v, --vault <VAULT>    Path to vault (overrides config)
  -h, --help             Print help
```
