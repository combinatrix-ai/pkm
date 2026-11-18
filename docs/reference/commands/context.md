# pkm context

```text
Get comprehensive context for a file or a '#tag' (for LLMs)

Usage: pkm context [OPTIONS] <FILE|#TAG>

Arguments:
  <FILE|#TAG>  A note, or a '#tag' to read as a virtual topic

Options:
  -c, --config <CONFIG>            Path to config file
      --no-content                 Exclude file content
      --no-backlinks               Exclude backlinks (faster)
  -v, --vault <VAULT>              Path to vault (overrides config)
      --no-tasks                   Exclude tasks
      --max-content <MAX_CONTENT>  Truncate content to N characters
      --line <N>                   Focus on physical 1-based line N
  -B, --before <N>                 Lines before the focused line (default: 3)
  -A, --after <N>                  Lines after the focused line (default: 3)
      --section                    Show the smallest enclosing Markdown section
      --outline-depth <N>          Focused document map depth (default: 1; 0 hides it)
  -f, --format <FORMAT>            [default: text]
  -h, --help                       Print help
```
