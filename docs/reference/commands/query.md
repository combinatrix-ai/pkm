# knapper query

```text
Filter notes by frontmatter, inline fields, and link counts

Usage: knapper query [OPTIONS]

Options:
  -c, --config <CONFIG>  Path to config file
  -w, --where <WHERE>    Filter, e.g. status=open, inlinks>3, tags~x
      --field <FIELD>    Extra column to show (repeatable)
  -v, --vault <VAULT>    Path to vault (overrides config)
      --from <FROM>      Only notes under this path
      --sort <SORT>      Sort by a field, e.g. inlinks:desc
  -l, --limit <LIMIT>    
  -f, --format <FORMAT>  [default: text]
  -h, --help             Print help
```
