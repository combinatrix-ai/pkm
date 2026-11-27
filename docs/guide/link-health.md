# Broken links include headings and block IDs

A link is only healthy when its complete destination exists. Resolving the
note is not enough when the link names an anchor: pkm also checks that the
heading or Obsidian block ID exists in the resolved Markdown note.

| Form | What is checked |
|---|---|
| `[[#Install]]` | the `Install` heading in the current note |
| `[[Guide#Install]]` | the `Install` heading in `Guide.md` |
| `[[#^build-step]]` or `[[^build-step]]` | the `^build-step` block ID in the current note |
| `[[Guide#^build-step]]` or `[[Guide^build-step]]` | the block ID in `Guide.md` |
| `[Install](#Install)` | a local Markdown heading anchor |
| `[Install](Guide.md#Install)` | a Markdown heading anchor in another note |

Missing anchors are ordinary broken-link findings with precise reasons:

```json
{
  "source": "Notes/Start here.md",
  "line": 12,
  "column": 5,
  "syntax": "wikilink",
  "raw": "[[Guide#Install]]",
  "raw_target": "Guide#Install",
  "target": "Guide#Install",
  "reason": "missing-heading",
  "status": "unresolved",
  "candidates": [],
  "note": "the heading does not exist in Notes/Guide.md"
}
```

The reason is `missing-heading` or `missing-block`. `broken-links --format
json` reports the occurrence, `lint --check broken-links` counts it, and
`query --where 'broken>0'` includes it in the source note's `broken` count.
`repair-links` reports it but offers no edit: the filesystem cannot infer which
heading or block the author intended.

Heading comparison is case-insensitive, percent-decoded, and ignores optional
closing `#` decoration (`## Install ##`). ATX and setext headings are indexed.
Block IDs are read from the end of prose lines. Frontmatter, fenced and inline
code, and `%%Obsidian comments%%` do not declare anchors. An escaped alias
separator in a Markdown table (`[[Guide#Install\|setup]]`) is handled as an
alias separator, not as part of the heading.

An anchor-only link stays local navigation and does not create a self-edge in
the link graph. A path-qualified anchor still creates the ordinary edge to its
note; if the anchor is absent, the same occurrence is additionally broken.

A short wikilink such as `[[Thesis]]` can resolve to `Notes/Thesis.md`.
If `[[Wrong/Thesis.md]]` has no matching folder path, lint reports a broken
link. It contributes no backlink to `Notes/Thesis.md` and stays untouched
when that note is renamed.
