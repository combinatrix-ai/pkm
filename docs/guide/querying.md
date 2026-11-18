# Querying notes

Filter and rank notes by their own fields and by their place in the link graph, and read or write frontmatter from the shell.

## One filter over every note

`orphans` and `hubs` are preset queries. `broken-links` adds occurrence
details and applies lint policies by default. `query` is the general form, so the combinations nobody wrote a command
for are available:

```bash
pkm query --where inlinks=0            # what orphans does
pkm query --sort inlinks:desc --limit 10   # what hubs does
pkm query --where 'broken>0' --field broken  # notes with broken links, without lint policies
```

The point is what those presets could not express — filtering on a note's own
fields and ranking by a computed one, in the same question:

```
$ pkm query --from Questions --where status=open --field inlinks --sort inlinks:desc
path                          inlinks
----------------------------  -------
Questions/where-to-live.md    4
Questions/which-bank.md       1
```

Frontmatter keys, Dataview inline fields and computed fields share one
namespace, so `status=open`, `cost>40` and `inlinks=0` all work the same way.
Operators are `=` `!=` `>` `<` `>=` `<=` `~` (contains), a bare name for
"has this field" and `!name` for "does not". `pkm fields` lists what a
given vault offers.

## Frontmatter

```bash
pkm frontmatter get Notes/Example.md --format json
pkm frontmatter get Notes/Example.md status
pkm frontmatter set Notes/Example.md status active
pkm frontmatter set Notes/Example.md score 3
pkm frontmatter set Notes/Example.md tags '[research, draft]'
pkm frontmatter delete Notes/Example.md score
```

`get` reads the whole YAML mapping or one key. `set` parses its value as YAML,
so numbers, booleans and lists retain their types; quote a YAML string inside
the shell argument when a scalar must remain a string. `delete` removes one
key. These write commands have no dry-run flag.

Tags are covered in [Hard links and soft tags](links-and-tags.md): `pkm tags`
lists them and `pkm tags --find NAME` finds the notes that carry one.
