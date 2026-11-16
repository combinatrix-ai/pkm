# Tasks

Use explicit `YYYY-MM-DD` values for task date filters and creation flags.
`tasks new` uses `--file` when supplied, otherwise `tasks.default_file`.
Use `--file` to scope status changes and `--partial` only when substring
matching is intended. The full task filter and subcommand options appear
in the [`knapper tasks` reference](../reference/commands/tasks.md).

Tasks are plain markdown checkboxes. The emoji date convention popularised by
the [Obsidian Tasks](https://publish.obsidian.md/tasks/) plugin (`📅` due,
`⏳` scheduled, `🛫` start, `✅` done, `🔁` recurring) is read and written but never
required — a vault of bare `- [ ]` items works fine.

```bash
knapper tasks --overdue                     # past due
knapper tasks --available                   # no start date, or start date <= today
knapper tasks --due-from 2026-08-01 --due-to 2026-08-31
knapper tasks --start-on 2026-08-01
knapper tasks --scheduled-from 2026-08-01 --scheduled-to 2026-08-07
knapper tasks --exclude Archive/ --tag work
knapper tasks new "write the README" --start 2026-07-28 --due 2026-08-01
knapper tasks done "write the README"       # exact match; --partial for substring
knapper tasks done "pay rent" --no-recur    # complete without the next occurrence
knapper tasks --prose-only                  # skip checkboxes in fenced examples
```

Every checkbox counts by default, including one written inside a code fence or
a `%%comment%%` — the one place knapper does not mask non-prose, and
deliberately: masking can only ever *hide* a task, and a visible example costs
less than a missing entry on a list you act on. `--prose-only` is the opt-out
for a vault that documents its own conventions.

Completing a recurring task with `tasks done` (or `tasks set done`) writes its
next occurrence on the line above, as the Tasks plugin does. `🔁 every N
days|weeks|months|years` advances from the due date (else scheduled, else
start); `when done` advances from today. The other dates keep their offset,
the created date becomes today, and a month-end date clamps (Jan 31 → Feb 28).
Richer rules such as `every week on Monday` are left alone with a warning;
`--no-recur` skips the next occurrence.

`--available` is also opt-in: it hides only tasks whose `🛫` start date is in
the future. Tasks without a start date remain visible, and plain `knapper
tasks` keeps its established whole-list behaviour. A scheduled date says when
you intend to work; it does not make a task unavailable.

Statuses are configurable: `open` `[ ]`, `wip` `[/]`, `done` `[x]`, and
`cancel` `[-]` are built in, and `knapper.yaml` can override their
markers or add custom statuses, which `knapper tasks set STATUS TEXT` and
`--status` filters then understand.
