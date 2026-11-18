# Hard links and soft tags

```bash
pkm tags                        # list tags across the vault
pkm tags Notes/Example.md       # list tags in one note
pkm tags --find research        # find notes carrying a tag
pkm backlinks '#research'       # inspect occurrences
pkm context '#research' --format json
```

A vault that has been written in for years is full of `[[book-club]]` — square
brackets used as a highlighter, never as a promise that a note by that name
exists. pkm is right to call those broken links, and the fix is not to
soften the report. It is to write what was meant.

So pkm distinguishes two kinds of reference:

| | means | missing target | in the graph |
|---|---|---|---|
| `[[X]]` | a **hard note reference** | a broken link | an edge |
| `#X` | a **soft topic reference** | fine, that is the point | nothing |

A tag is not a note and never becomes one. It is not an orphan, not a hub, and
`broken-links` has nothing to say about it. But it is still navigable, because
a label you cannot follow is only half a label:

```
$ pkm backlinks '#book-club'

Daily/2026-07-01.md (line 10) #book-club

Daily/2026-07-01.md (line 22) #book-club

Notes/Events.md (line 3) #book-club/sessions
```

Quote it — an unquoted `#` starts a comment in most shells. A leading `#` is
the **only** thing that selects a tag, so nothing changes for ordinary
arguments, and no tag becomes a graph node by accident. It is also
unconditional: a note whose filename really does start with `#` is reached by
writing a path for it, `./#notes.md`.

`context` takes one too, and answers with the whole subject rather than with a
note that does not exist:

```bash
pkm context '#book-club' --format json
```

```json
{
  "kind": "tag",
  "tag": "book-club",
  "selector": "#book-club",
  "notes": ["Daily/2026-07-01.md", "Notes/Events.md", "Notes/Log.org"],
  "nested_tags": ["book-club/sessions"],
  "occurrences": [
    {
      "source": "Daily/2026-07-01.md",
      "line": 10,
      "tag": "book-club",
      "where": "inline",
      "text": "- Wrote the reading list #book-club"
    }
  ],
  "tasks": [{"file": "Daily/2026-07-01.md", "line": 22, "text": "Schedule a session #book-club", "done": false}],
  "stats": {"notes": 3, "occurrences": 4}
}
```

There is no `path` and no `content`, because there is no note. `kind` is there
so a caller never has to guess which shape it got, and note `context` is
unchanged. Of `context`'s flags, `--no-content` drops each occurrence's line
text, `--max-content` truncates it and `--no-tasks` drops the tasks;
`--no-backlinks` has nothing to skip, since the occurrences *are* the
backlinks.

The matching rules are deliberately narrow, and the same in both commands:

- A tag covers the tags **nested** under it — `#work` finds `#work/events`,
  which is what writing a nested tag means. Every result names the tag it
  actually matched, so an exact hit is distinguishable from a nested one.
- Matching is otherwise **exact and case-sensitive**, with no Unicode
  normalisation. That is the rule `tags --find` already follows, so one vault
  cannot have two answers to "which notes carry this tag".
- An occurrence is either **inline** (`#tag` in the prose) or **declared**
  (YAML `tags:`, org `#+filetags:` and heading tags) — the same topic in the
  syntax each flavor uses for it, and `where` says which.
- Code fences, inline spans and `%%comments%%` hold no occurrences, exactly as
  they hold no links.
- Results are ordered by path, then line, so the JSON is stable between runs.
- `-A`/`-B` work as they do for note backlinks.

## Demoting a wikilink to a tag

`demote` is the migration: it rewrites the **exact** form and nothing else.

```
$ pkm demote "book-club" --dry-run
[DRY RUN] Demoting [[book-club]] -> #book-club
  Would update 2 links in 2 files:
    Daily/2026-07-01.md
      9: [[book-club]] -> #book-club
    Daily/2026-07-02.md
      7: [[book-club]] -> #book-club
  ⚠️ Daily/2026-07-01.md:15: [[book-club|club discussion]] cannot be demoted (alias)
  ⚠️ Daily/2026-07-01.md:19: [[Archives/book-club]] cannot be demoted (path-qualified)
  ⚠️ Daily/2026-07-01.md:20: [[book-club]] cannot be demoted (adjacent text)

Nothing was written.
```

Everything a tag cannot hold is reported rather than mangled — and rather than
passed over in silence, because those are the ones still to deal with by hand:

| reason | example |
|---|---|
| `alias` | `[[X\|the topic]]` — a tag has nowhere to keep display text |
| `anchor` | `[[X#Heading]]`, `[[X^b12]]` — a tag has no inside |
| `embed` | `![[X]]` — an embed transcludes, a tag does not |
| `path-qualified` | `[[Folder/X]]` — a path names a file, not a topic |
| `adjacent text` | `saw[[X]]again` — `#Xagain` would be a *different* tag |
| `frontmatter` | a YAML value is not prose, and `#X` in one is a string |
| `org-mode` | pkm reads org and does not rewrite it, here as elsewhere |

Code fences, inline code and `%%comments%%` are neither rewritten nor
reported: there are no references in them to report. Excluded subtrees are
never touched. Only the matched span is replaced, so indentation, the rest of
the line, the trailing newline and CRLF endings all survive as they were.

`--dry-run` is available and writes nothing, exactly as for `rename` and
`move`; `--format json` gives the whole plan:

```json
{
  "kind": "demote",
  "target": "book-club",
  "tag": "book-club",
  "dry_run": true,
  "applied": false,
  "files_updated": ["Daily/2026-07-01.md", "Daily/2026-07-02.md"],
  "links_updated": 2,
  "edits": [{"file": "Daily/2026-07-01.md", "links": 1,
             "changes": [{"line": 9, "before": "[[book-club]]", "after": "#book-club"}]}],
  "skipped": [{"file": "Daily/2026-07-01.md", "line": 15,
               "text": "[[book-club|club discussion]]", "reason": "alias"}]
}
```

`edits` and `skipped` are present in both modes and ordered by file then line.
Three things it refuses outright, before writing anything:

- A target that **names a note that exists**. That is not a relabelling, it is
  a deletion: the reference stops pointing anywhere and the note loses a
  backlink. `--allow-existing-note` does it anyway.
- A target that **cannot be spelled as a tag** — `Daily Tasks` has a space,
  `2026` has no letter. Pass `--tag work/daily` to name the tag to write.
- A **path-qualified** target. `demote` names a bare target; to write a nested
  tag, name the bare target and pass `--tag a/b`.

A target nothing links to is an error rather than a silent zero, because at
that point it is a typo far more often than it is a no-op.

Promotion the other way — `#X` back to `[[X]]` — is **intentionally not
implemented**. It is not the symmetric operation it looks like: demotion
strictly weakens a reference and needs nothing to exist, while promotion has
to invent a note, decide where it lives, and turn a label into a promise that
the vault then has to keep. A nested `#a/b` has no single note name at all,
and a tag on fifty daily notes would produce fifty new hard links to one new
file. Create the note and use `rename`, which already does that safely.
