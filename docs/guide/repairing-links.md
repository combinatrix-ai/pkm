# Repairing links that broke somewhere else

`rename` and `move` keep links intact through a refactor knapper performs.
`repair-links` is the other half: the links that broke while knapper was not
looking — a folder reorganised in Finder, an exporter that wrote a path which
has since moved, an encoding that survived one round trip too many.

```
$ knapper repair-links --dry-run
[DRY RUN] repair-links: 12 broken link occurrence(s) in 2 file(s)
  safe 6, ambiguous 2, unresolved 4

safe (6)
  Index.md:10:3  [[legacy/notes/Foo.md]] -> [[notes/Foo.md]]
      notes/Foo.md (unique-path-suffix: the only file whose path ends with "notes/Foo.md", and "notes/Foo.md" resolves back to it)
  Index.md:13:3  [[Docs/My%20Note]] -> [[Docs/My Note]]
      Docs/My Note.md (percent-decoding: decoding the target gives "Docs/My Note", and "Docs/My Note" resolves back to it)

ambiguous (2)
  Index.md:18:3  [[stale/docs/README.md]]
      missing-path: 2 files end with "docs/README.md"
      - Handbook/docs/README.md
      - Manual/docs/README.md

unresolved (4)
  Index.md:22:3  [[Roam]]
      missing-note: no file matches this target

Nothing was written. repair-links plans repairs; it never edits the vault.
```

**V1 plans and nothing else.** `--dry-run` is required — omitting it is
refused with exit 2 before the vault is even read, so a forgotten flag can
never be mistaken for asking to write. There is no `--apply` yet.

## What counts as evidence

A repair is proposed only where the filesystem settles it. Every occurrence
lands in one of three buckets:

| | means |
|---|---|
| `safe` | exactly one destination, reached by an exact structural transformation, and the link knapper would write resolves back to it |
| `ambiguous` | something matches, but choosing between the matches is a judgement about meaning |
| `unresolved` | nothing on disk could settle it |

Two bases are safe, and they are both about structure rather than about
resemblance:

- **`unique-path-suffix`** — the tail of the path is still exactly right and
  only the directories in front of it are stale. `[[legacy/notes/Foo.md]]`
  when `notes/Foo.md` is the *only* file whose path ends `notes/Foo.md`.
  Leading `../` is treated as part of the stale prefix, not followed.
- **`percent-decoding`** — decoding the target yields the link that was meant,
  and the decoded text **names a path** that exists. `[[Docs/My%20Note]]` for
  `Docs/My Note.md`.

Decoding is not a doorway to anything looser. knapper resolves a bare name by
stem and by alias when it *reads* a vault, and neither is evidence for an
edit — so a decoded target is only safe when it names its destination as a
path. `[[My%20Note]]` decodes onto the very same `Docs/My Note.md`, but gets
there by filename, and is only a suggestion.

Everything else is a suggestion at most:

- **`basename`** — one component matched and the directory did not.
  `[[old/Bar/Foo.md]]` beside `notes/Foo.md` is a resemblance, not a path, and
  so is `[[My%20Note]]` beside `Docs/My Note.md`.
- **An alias.** A name the author gave a note is not a location, so a target
  that reaches a note only through its `aliases:` stays `unresolved`.
- **`path-suffix` with more than one hit** — two files end the same way, so
  nothing is proposed and both are listed.
- **A renamed concept.** `[[Roam]]` next to a note called `RoamResearch` is a
  rename somebody performed in their head. No string distance turns that into
  evidence, and knapper does not try; it stays `unresolved`.
- **Missing dates and citation labels.** `[[2026-07-04]]` and `[[12]]` name no
  note at all, so there is nothing to repair and nothing is invented.
- **org links.** knapper reads org and does not rewrite it, here as in `move`
  and `demote`, so an org occurrence is reported with its line and never with
  a plan.

One basis is deliberately absent: replacing a *wrong* extension. Note
extensions need no repair — the resolver answers `[[notes/Foo.md]]` and
`[[notes/Foo]]` alike — so all such a rule could add is dropping an extension
that is not a note's, and `old/Foo.txt` becoming `notes/Foo.md` is a claim
about what the author meant by `.txt`.

Two rules keep it honest in the other direction. A link that already
**resolves is never a repair** — including a valid note-relative link, which
is not rewritten to vault-root form for style. And **excluded notes and
non-note leaves are valid destinations**, exactly as they are for the
resolver: `Archive/Old Plan.md` and `assets/paper.pdf` are real files a link
may point at.

Every safe proposal is checked by resolving the text knapper would write — as
a path, again, so a rewrite whose correctness depended on no other note ever
taking that name is not proposed. Nothing is published on the strength of a
search a reader would not reproduce.
Destinations always come from the vault walk, which does not descend symlinked
directories, so nothing behind one can become a repair target.

## The plan, as JSON

```bash
knapper repair-links --dry-run --format json
```

```json
{
  "kind": "repair-links",
  "dry_run": true,
  "applied": false,
  "summary": {"files": 2, "occurrences": 12, "safe": 6, "ambiguous": 2, "unresolved": 4},
  "occurrences": [
    {
      "source": "Index.md",
      "line": 10,
      "column": 3,
      "syntax": "wikilink",
      "raw": "[[legacy/notes/Foo.md]]",
      "raw_target": "legacy/notes/Foo.md",
      "target": "legacy/notes/Foo.md",
      "reason": "missing-path",
      "status": "safe",
      "candidates": [
        {"path": "notes/Foo.md", "basis": "unique-path-suffix", "matched": "notes/Foo.md"}
      ],
      "note": null,
      "edit": {
        "byte_start": 124,
        "byte_end": 147,
        "before": "[[legacy/notes/Foo.md]]",
        "after": "[[notes/Foo.md]]",
        "target_before": "legacy/notes/Foo.md",
        "target_after": "notes/Foo.md",
        "resolves_to": "notes/Foo.md"
      }
    }
  ]
}
```

- `status` is `safe`, `ambiguous` or `unresolved`; `reason` is `missing-note`,
  `missing-path`, `missing-date`, `missing-heading`, `missing-block`,
  `numeric-label` or `org-link`. A missing heading or block ID is reported but
  never offered a repair candidate: the filesystem cannot infer its intended
  destination.
- `column` is 1-based and counted in **characters**, so a line of Japanese
  reports the column a reader would count. It is `null` for org, whose masking
  does not preserve byte offsets — the line is exact, the column is not
  available, and no org occurrence carries an edit.
- `edit` carries the byte span plus the text before and after, which is
  everything an apply needs without scanning the vault again. It is `null`
  wherever `status` is not `safe`.
- Occurrences are ordered by source, then line, then column, so two runs over
  an unchanged vault produce identical bytes.

`broken-links --all --format json` is the same records without the plan — one
object per occurrence, with the position, the text as written and the
candidates. Without `--all`, `broken-links` applies the same policies as
ordinary lint. `query --where 'broken>0'` remains independent of lint policies.

One consequence worth knowing: knapper resolves a path-qualified link by
basename when the path itself misses, so `[[legacy/notes/Foo]]` to a live note
already resolves and never appears here. What does appear is the case that
basename fallback cannot rescue — a path written with its extension, or one
whose leaf is an excluded note or a non-note file.
