# pkm query

```text
Filter notes by frontmatter, inline fields, and link counts

Usage: pkm query [OPTIONS]

Options:
  -c, --config <CONFIG>    Path to config file
      --tag <TAG>          Require this exact tag (repeat to require all)
      --exclude-tag <TAG>  Exclude results with this exact tag (repeat to exclude any)
  -v, --vault <VAULT>      Path to vault (overrides config)
  -w, --where <WHERE>      Filter, e.g. status=open, inlinks>3, tags~x
      --field <FIELD>      Extra column to show (repeatable)
      --from <FROM>        Only notes under this path
      --sort <SORT>        Sort by a field, e.g. inlinks:desc
  -l, --limit <LIMIT>
  -f, --format <FORMAT>    [default: text]
  -h, --help               Print help
```
