# pkm repair-links

```text
Plan repairs for links that are already broken.

This is the other half of `rename` and `move`: they keep links intact through a
refactor pkm performs, and this reports the ones that broke while pkm was
not looking -- a folder reorganised elsewhere, an exporter's stale path.

A repair is proposed only where the filesystem settles it: exactly one file, reached
by an exact structural transformation of the target as written. A resemblance is
not evidence, so a renamed concept, a missing date and a citation label are reported
and left alone.

V1 is read-only. --dry-run is required, and nothing is ever written.

Usage: pkm repair-links [OPTIONS]

Options:
  -c, --config <CONFIG>
          Path to config file

      --dry-run
          Required: this command only ever plans

  -f, --format <FORMAT>
          [default: text]

  -v, --vault <VAULT>
          Path to vault (overrides config)

  -h, --help
          Print help (see a summary with '-h')
```
