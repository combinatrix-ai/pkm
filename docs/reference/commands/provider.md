# knapper provider

```text
Configure the commands that resolve references

Usage: knapper provider [OPTIONS] <COMMAND>

Commands:
  list    List the configured providers and the commands they run
  set     Define or replace one provider's command
  remove  Forget one provider
  help    Print this message or the help of the given subcommand(s)

Options:
  -c, --config <CONFIG>  Path to config file
  -v, --vault <VAULT>    Path to vault (overrides config)
  -h, --help             Print help
```

## knapper provider list

```text
List the configured providers and the commands they run

Usage: knapper provider list [OPTIONS]

Options:
  -c, --config <CONFIG>  Path to config file
  -f, --format <FORMAT>  [default: text]
  -v, --vault <VAULT>    Path to vault (overrides config)
  -h, --help             Print help
```

## knapper provider set

```text
Define or replace one provider's command

Usage: knapper provider set [OPTIONS] <NAME> -- <COMMAND>...

Arguments:
  <NAME>        
  <COMMAND>...  Command after `--`, e.g. -- op read 'op://Vault/{locator}/value'

Options:
  -c, --config <CONFIG>  Path to config file
  -v, --vault <VAULT>    Path to vault (overrides config)
  -h, --help             Print help
```

## knapper provider remove

```text
Forget one provider

Usage: knapper provider remove [OPTIONS] <NAME>

Arguments:
  <NAME>  

Options:
  -c, --config <CONFIG>  Path to config file
  -v, --vault <VAULT>    Path to vault (overrides config)
  -h, --help             Print help
```
