# Working with agents

"Agent-friendly" is cheap to say. Concretely: every query command speaks
`--format json`, the list-producing ones add `--format paths`, and the output
is made to be piped:

```bash
knapper backlinks "Projects/recipe.md" --format json | jq -r '.[].source'
knapper broken-links --format paths
knapper frontmatter set "Projects/x.md" status done
```

`knapper links FILE` lists a note's outgoing links and `knapper backlinks FILE`
its incoming ones; both take `-A`/`-B` for surrounding lines.

And `context` aggregates what a model would otherwise need five calls to
gather — content, outgoing links, backlinks, tags, headings, stats:

```bash
knapper context "Notes/Lit Review.md"
```

```json
{
  "path": "Notes/Lit Review.md",
  "title": "Lit Review",
  "content": "# Lit Review\n\nRelated: ...",
  "links": ["Missing Note", "Projects/Cookbook"],
  "backlinks": ["Projects/Cookbook.md"],
  "tags": ["parent/child", "research"],
  "headings": [{"level": 1, "text": "Lit Review", "line": 1}],
  "stats": {"chars": 95, "words": 10, "lines": 5}
}
```

To restore only the neighbourhood of a search hit, pass its physical
1-based line. Focused context defaults to three lines before and after the
hit, and includes a compact first-layer document map plus the complete heading
breadcrumb:

```bash
knapper context "Notes/Lit Review.md" --line 9
knapper context "Notes/Lit Review.md" --line 9 -B 1 -A 8 --format json
knapper context "Notes/Lit Review.md" --line 9 --section
```

`--section` selects the smallest enclosing Markdown section and cannot be
combined with explicit `-A/--after` or `-B/--before`. `--outline-depth N`
controls the map (`1` by default; `0` hides it). The map shows at most twelve
entries; larger outlines keep the first, current-neighbour and last entries
and report `total`, `omitted` and `truncated`. An H1 is the document title and
its direct children are the first layer; a single H1 without children is only
the title. When there are multiple H1s they are peer first-layer roots with no
document title; without an H1, the shallowest heading level is the first layer.
Focused mode is Markdown-only for now and rejects org-mode
notes rather than presenting a misleading outline.

Focused JSON is deterministic and has this shape:

```json
{
  "kind": "note",
  "path": "Notes/Lit Review.md",
  "document": {
    "title": "Lit Review",
    "outline_depth": 1,
    "entries": [{"index": 0, "title": "Method", "depth": 1, "start_line": 9, "end_line": 11, "current": true}],
    "total": 1, "omitted": 0, "truncated": false, "current_branch_index": 0
  },
  "focus": {
    "line": 9,
    "heading_path": ["Lit Review", "Method"],
    "enclosing_section": {"start_line": 9, "end_line": 11}
  },
  "excerpt": {"start_line": 6, "end_line": 11, "truncated": false, "content": "..."}
}
```

`--no-content` retains all focus, section and outline metadata while omitting
`excerpt.content`. `--max-content` truncates around the focused line in this
mode, rather than returning the beginning of the file; `excerpt.content` stays
source-only and `truncated: true` causes text output to print an unnumbered
truncation marker.

## The skill ships inside the binary

An agent host does not need this repository to learn how to drive knapper:

```bash
knapper skill              # print it
knapper skill --install    # write it into ~/.claude and ~/.codex
```

`install.sh` registers it for you, and `knapper self-update` re-registers it,
so the instructions an agent reads cannot drift behind the binary they
describe. The source is [`assets/knapper-skill.md`](../../assets/knapper-skill.md).
