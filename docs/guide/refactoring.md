# Renames that don't break anything

The command that earns the install. Rename or move a note, and every inbound
link is rewritten — wikilinks and inline links alike:

```
$ knapper rename "Recipe" "Cookbook"
Renaming: Projects/Recipe.md -> Projects/Cookbook.md
Found 2 files with links to update
  Updated 1 links in Notes/Experiment Plan.md
  Updated 1 links in Notes/Lit Review.md

Done! Renamed and updated 2 links in 2 files.
```

```diff
 # Notes/Lit Review.md
-Related: [[Projects/Recipe]].
+Related: [[Projects/Cookbook]].

 # Notes/Experiment Plan.md
-Links back to [the recipe](../Projects/Recipe.md).
+Links back to [the recipe](../Projects/Cookbook.md).
```

Link text, folder prefixes, `#anchors`, and `%20` percent-encoding are all
preserved; external URLs and image embeds are left alone. Preview any refactor
with `--dry-run`:

```bash
knapper rename "old-name" "new-name" --dry-run
knapper move "note.md" "Archive/" --dry-run
```

## Moving a whole directory

`move` takes a directory as well as a note. The subtree moves as it stands —
notes, attachments, sidecar files, dotfiles, nested folders, byte for byte —
and every link that *resolves* into it is rewritten:

```
$ knapper move "Projects/Recipe" "Archive/" --dry-run
[DRY RUN] Moving directory: Projects/Recipe -> Archive/Recipe
  9 files (4 notes) move with it
  Would update 6 links in 3 files:
    Notes/Lit Review.md
      12: [[Projects/Recipe/README]] -> [[Archive/Recipe/README]]
      14: ![plan](Projects/Recipe/assets/plan.png) -> ![plan](Archive/Recipe/assets/plan.png)
    ...

Nothing was written.
```

The source is matched exactly — a vault-relative path, with or without a
trailing slash — and the result is always `DEST/<the directory's own name>`.
It does not rename the directory, and it does not merge into an existing one.

"Every inbound link" means every link that *points at a file in the subtree*,
not every line that mentions the path. So a bare `[[README]]` written next to
a different README is left exactly as it is, while one that meant the moved
README is qualified so it still lands there. Links inside the subtree are
untouched when the structure keeps them working, and a relative link out of
the subtree is recomputed for its new depth. Image and attachment links follow
the files they point at.

The whole plan is built before anything is written — so `--dry-run` shows the
real one, and a write that fails part-way restores what it touched:

```bash
knapper move "Projects/Recipe" "Archive/" --format json
```

```json
{
  "kind": "directory",
  "old_path": "Projects/Recipe",
  "new_path": "Archive/Recipe",
  "applied": true,
  "entries": 9,
  "notes": 4,
  "files_updated": ["Notes/Lit Review.md"],
  "links_updated": 6,
  "unsupported_links": [],
  "warnings": []
}
```

`--dry-run` adds `moves` (every file's old and new path) and `edits` (every
link change, with its line, before and after), which is enough to act on
without reading the vault again.

Both ends have to be inside the real vault tree. A source or destination that
reaches its directory through a **symlink** is refused, even when the path
looks perfectly vault-relative: `Archive/` could be a link to anywhere, and
following it would move the directory somewhere knapper can no longer see
while rewriting every inbound link to a path that no longer exists.

That is about the directories being moved. A symlinked *note* is a different
thing and is followed, as it is everywhere else in knapper: a link rewritten
inside one changes the file the link points at, and leaves the link itself a
link.

Rewritten inline links are percent-encoded for the path they now have, not
for the one they had. A directory called `Guide (v2)` produces
`[a](Archive/Guide%20%28v2%29/Note.md)`, because an unencoded `)` would end
the link at the wrong place — and the same goes for `%`, `"`, `<`, `>` and a
`#`, which would otherwise be read as the anchor. Wikilinks need none of this
and get none of it; a `<...>` destination keeps its angle brackets.

One gap it will not paper over: knapper reads org-mode but does not rewrite
it. If an inbound `.org` link points into the directory, the move stops,
reports the links under `unsupported_links`, and changes nothing —
`--allow-broken-org-links` does it anyway and still reports them.
